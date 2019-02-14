mod auth;
mod error;
mod opt;

use egg_mode::media::{media_types, UploadBuilder};
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;
use error::*;
use mime::Mime;
use opt::Opts;
use rand::seq::IteratorRandom;
use std::ffi::OsStr;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::runtime::current_thread::block_on_all;

fn main() {
    let opts = Opts::from_args();

    let result = match opts {
        Opts::Auth(args) => auth(args),
        Opts::Tweet(args) => tweet(args),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);

        for cause in err.iter().skip(1) {
            eprintln!("Caused by: {}", cause);
        }

        std::process::exit(1);
    }
}

fn auth(opts: opt::Auth) -> Result<()> {
    let consumer = egg_mode::KeyPair::new(opts.consumer_key, opts.consumer_secret);

    match auth::get_access_token_sync(&consumer)? {
        Token::Access { consumer, access } => {
            println!("export CONSUMER_KEY='{}'", consumer.key);
            println!("export CONSUMER_SECRET='{}'", consumer.secret);
            println!("export ACCESS_TOKEN='{}'", access.key);
            println!("export ACCESS_TOKEN_SECRET='{}'", access.secret);
        }
        Token::Bearer(bearer) => {
            println!("Bearer: {}", bearer);
        }
    }

    Ok(())
}

fn tweet(opts: opt::Tweet) -> Result<()> {
    let (file_path, mime_type) = get_media_files(&opts.path)
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| Error::context("failed to select media file"))?;

    println!("Reading file: {}", &file_path.to_string_lossy());
    let file_bytes = std::fs::read(&file_path).context(|| "unable to read file")?;

    let mut decoded_text = file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(|| Error::context("file name was not valid UTF-8"))
        .and_then(|name| {
            base64::decode_config(name, base64::URL_SAFE).context(|| "failed to decode filename")
        })
        .and_then(|decoded| {
            String::from_utf8(decoded).context(|| "decoded filename was not valid UTF-8")
        })
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            "".to_string()
        });

    while decoded_text.len() > opts.max_length {
        decoded_text.pop();
    }

    let consumer = egg_mode::KeyPair::new(opts.consumer_key, opts.consumer_secret);
    let access = egg_mode::KeyPair::new(opts.access_token, opts.access_token_secret);
    let token = egg_mode::Token::Access { consumer, access };

    println!("Uploading file");
    let builder = UploadBuilder::new(file_bytes, mime_type).alt_text(&decoded_text);

    let media_handle =
        block_on_all(builder.call(&token)).context(|| "failed to get media handle")?;
    let draft = DraftTweet::new(&decoded_text).media_ids(&[media_handle.id]);

    println!("Posting tweet: {}", &decoded_text);
    block_on_all(draft.send(&token)).context(|| "failed to post tweet")?;

    if opts.delete {
        std::fs::remove_file(&file_path).context(|| "failed to delete file")?;
    }

    Ok(())
}

fn get_media_files(path: &str) -> impl Iterator<Item = (PathBuf, Mime)> {
    let iter = walkdir::WalkDir::new(path).follow_links(true).into_iter();

    let media_files = iter.filter_map(|e| match e {
        Err(_) => None,
        Ok(ref entry) if entry.file_type().is_dir() => None,
        Ok(ref entry) => {
            let extension = entry.path().extension().and_then(OsStr::to_str);
            if extension.is_none() {
                return None;
            }

            let media_type = match extension.unwrap() {
                "jpg" | "jpeg" => media_types::image_jpg(),
                "png" => media_types::image_png(),
                "webp" => media_types::image_webp(),
                "mp4" => media_types::video_mp4(),
                _ => return None,
            };

            Some((entry.path().to_path_buf(), media_type))
        }
    });

    media_files
}
