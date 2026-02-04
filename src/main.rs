//! #sendTelegram
//!
//! A small tool to send messages to a private Telegram chat.
//!
pub mod cfgfile;
pub mod cmdargs;

use curl::easy::Easy;
use log::{debug, warn};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::collections::HashMap;
use std::io::{stdout, Read, Write};

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .without_timestamps()
        .init()
        .unwrap();

    let mut opts = HashMap::<String, String>::new();
    opts.insert(
        String::from("cfgfile"),
        String::from("/etc/sendtelegram.cfg"),
    );
    opts.insert(String::from("msg"), String::from("I wanna chat ..."));

    cmdargs::parse(&mut opts);

    cfgfile::parse(&mut opts);
    // print the options struct using the debug trait
    debug!("Opts: {opts:?}");

    let mut data = String::from("chat_id=");
    data.push_str(opts.get("CHAT").expect("Error, no CHAT ID"));
    data.push_str("&disable_notification=false");
    data.push_str("&text=");
    // match options.icon {
    match opts.get("icon") {
        // if we have an icon, add it
        Some(icon) => data.push_str(icon),
        None => (), // else do nothing
    }
    // escape and add message
    data.push_str(
        &percent_encode(
            opts.get("msg")
                .expect("Error, no message to send.")
                .as_bytes(),
            NON_ALPHANUMERIC,
        )
        .to_string(),
    );

    let mut url = String::from("https://api.telegram.org/bot");
    url.push_str(opts.get("API").expect("Error, no API key"));
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
        .read_function(|buf| Ok(data.as_bytes().read(buf).unwrap()))
        .unwrap();

    // callback to read response send by server
    transfer
        .write_function(|data| Ok(stdout().write(data).unwrap()))
        .unwrap();

    // if !options.nosend {
    match opts.get("nosend") {
        None => transfer.perform().unwrap(),
        Some(_) => warn!("Warning, not sending !"),
    }
    println!("");
}
