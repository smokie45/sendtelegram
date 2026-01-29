//! #sendTelegram
//!
//! A small tool to send messages to a private Telegram chat.
//!
//! TODO: use one dict / struct to keep opts. Forward to cmdargs and cfgfile
//!         - use hashmap, add keys from main
//!         - filearg will parse and add value on match
pub mod cfgfile;
pub mod cmdargs;

use curl::easy::Easy;
use log::{debug, info, warn};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::io::{stdout, Read, Write};

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .without_timestamps()
        .init()
        .unwrap();

    let options = cmdargs::parse().expect("Error, parsing cmdargs.");
    // print the options struct using the debug trait
    debug!("Options: {options:?}");

    let opts = cfgfile::parse(&options.cfgfile);
    debug!("Opts: {opts:?}");

    let mut data = String::from("chat_id=");
    data.push_str(&opts.chat);
    data.push_str("&disable_notification=false");
    data.push_str("&text=");
    match options.icon {
        // if we have an icon, add it
        Some(icon) => data.push(icon),
        None => (), // else do nothing
    }
    // escape and add message
    data.push_str(&percent_encode(options.msg.as_bytes(), NON_ALPHANUMERIC).to_string());

    let mut url = String::from("https://api.telegram.org/bot");
    url.push_str(&opts.api);
    url.push_str("/sendMessage");

    debug!("URL = '{}'", url);
    let mut easy = Easy::new();
    // let _ = easy.verbose(true);
    easy.url(&url).unwrap();
    // unwrap() will panic if error is returned, else it continues
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();

    // callback to read data to send to server
    transfer
        .read_function(|buf| {
            Ok(data.as_bytes().read(buf).unwrap())
            // Ok(data.read(buf).unwrap_or(0))
        })
        .unwrap();

    // callback to read response send by server
    transfer
        .write_function(|data| Ok(stdout().write(data).unwrap()))
        .unwrap();

    if !options.nosend {
        transfer.perform().unwrap();
    }
    println!("");
}
