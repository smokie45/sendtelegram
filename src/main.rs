//! #sendTelegram
//!
//! A small tool to send messages to a private Telegram chat.
//!
mod cfgfile;
mod cmdargs;

use curl::easy::Easy;
use log::{debug, warn};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::collections::HashMap;
use std::io::{stdout, Read, Write};

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .env()
        .without_timestamps()
        .init()
        .unwrap();

    // create a hasmap of strings to carry all options
    let mut opts = HashMap::<String, String>::new();
    // set some default options
    opts.insert(
        String::from("cfgfile"),
        String::from("/etc/sendtelegram.cfg"),
    );
    opts.insert(String::from("msg"), String::from("I wanna chat ..."));
    // parse cmdline arguments into the opts hashmap
    cmdargs::parse(&mut opts);
    // parse a TOML config file into the opts hashmap
    cfgfile::parse(&mut opts);
    // print the options struct using the debug trait
    debug!("Opts: {opts:?}");

    // prepare additional data for REST call
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

    // creat the URL for curl
    let mut url = String::from("https://api.telegram.org/bot");
    url.push_str(opts.get("API").expect("Error, no API key"));
    url.push_str("/sendMessage");
    debug!("URL = '{}'", url);

    // instanciate curl object
    let mut easy = Easy::new();
    match opts.get("verbose") {
        Some(_) => easy.verbose(true).unwrap(),
        None => (),
    }

    easy.url(&url).unwrap();
    // unwrap() will panic if error is returned, else it continues
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    // callback to read data for sendinf to server
    transfer
        .read_function(|buf| Ok(data.as_bytes().read(buf).unwrap()))
        .unwrap();
    // print curl return data to stdout, if verbose is set, else do nothing
    match opts.get("verbose") {
        Some(_) => transfer
            // callback to read response send by server
            .write_function(|data| Ok(stdout().write(data).unwrap()))
            .unwrap(),
        None => (),
    }

    // send only, if nosend is not set
    match opts.get("nosend") {
        None => transfer.perform().unwrap(),
        Some(_) => warn!("Warning, not sending !"),
    }
    println!("");
}
