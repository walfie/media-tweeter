use crate::error::*;
use egg_mode::KeyPair;
use std::io::BufRead;
use tokio::runtime::current_thread::block_on_all;

pub fn get_access_token_sync(consumer: &KeyPair) -> Result<egg_mode::Token> {
    let request_token = block_on_all(egg_mode::request_token(consumer, "oob"))
        .context(|| "failed to request PIN")?;

    // "oob" is needed for PIN-based auth
    let auth_url = egg_mode::authorize_url(&request_token);

    eprintln!("Please visit the following page: {}\n", auth_url);

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        eprint!("Enter the PIN: ");
        let pin = lines
            .next()
            .ok_or_else(|| Error::context("input closed"))?
            .context(|| "input error")?;

        let result = block_on_all(egg_mode::access_token(
            consumer.clone(),
            &request_token,
            pin.trim(),
        ));

        match result {
            Ok((token, _, _)) => return Ok(token),
            Err(e) => eprintln!("failed to get access token: {}", e),
        }
    }
}
