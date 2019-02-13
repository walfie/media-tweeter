use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "media-tweeter")]
pub enum Opts {
    #[structopt(name = "auth", about = "Get auth token with PIN")]
    Auth(Auth),

    #[structopt(name = "upload", about = "Upload random media")]
    Tweet(Tweet),
}

#[derive(Debug, StructOpt)]
pub struct Auth {
    #[structopt(long = "consumer-key", env = "CONSUMER_KEY")]
    pub consumer_key: String,
    #[structopt(long = "consumer-secret", env = "CONSUMER_SECRET")]
    pub consumer_secret: String,
}

#[derive(Debug, StructOpt)]
pub struct Tweet {
    #[structopt(long = "consumer-key", env = "CONSUMER_KEY")]
    pub consumer_key: String,
    #[structopt(long = "consumer-secret", env = "CONSUMER_SECRET")]
    pub consumer_secret: String,

    #[structopt(long = "access-token", env = "ACCESS_TOKEN")]
    pub access_token: String,
    #[structopt(long = "access-token-secret", env = "ACCESS_TOKEN_SECRET")]
    pub access_token_secret: String,

    #[structopt(long = "rm", help = "Delete the file after posting")]
    pub delete: bool,

    #[structopt(
        long = "max-length",
        help = "Max tweet length in bytes",
        default_value = "240"
    )]
    pub max_length: usize,

    #[structopt(
        help = "Path of file to upload. If the input is a directory, a media file will be selected randomly."
    )]
    pub path: String,
}
