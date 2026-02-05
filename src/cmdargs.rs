use std::collections::HashMap;
use std::env;
use std::io::{IsTerminal, Read};

/// Print help text describing all options
fn show_help() {
    println!("Send notifications to a telegram channel");
    println!();
    println!("usage: sendtelegram [options]");
    println!();
    println!("Options:");
    println!("          -m TEXT      The message to send");
    println!("          -c CATEGORY  A category /icon");
    println!("          -v           verbose");
    println!("          -n           Debug - don't send");
    println!("          -f           Configuration file (Default: /etc/sendtelegram.cfg)");
    println!();
    println!("Talking with Telegram requires a private API key and CHAT ID. Both shall be defined");
    println!("in a configuration file, by default expected at '/etc/sendtelegram.cfg'");
    println!("The message to send can also be provided via STDIN to sendtelegram.");
    println!("Use RUST_LOG=[info, debug, ...] env to enable logging.");
}

/// Parse all cmdline arguments and return a struct of options.
///
/// Convert a textual category argument into a UTF icon.
pub fn parse(opts: &mut HashMap<String, String>) {
    let mut icons = HashMap::new();
    icons.insert(String::from("Bell"), '🔔');
    icons.insert(String::from("Watch"), '⌚');
    icons.insert(String::from("HighVoltage"), '⚡');

    // check is a message is piped into the binary ...
    if !std::io::stdin().is_terminal() {
        let mut tmp = String::new();
        std::io::stdin().read_to_string(&mut tmp).expect("ERROR");
        opts.insert(String::from("msg"), tmp);
    }

    let mut args = env::args();
    args.next(); // skip the command itself
    while let Some(argument) = args.next() {
        // Note: We use while because it will not own the iter and allows us to advance it
        // manually. A for loop will own the iter and does not allow manual adavance.
        match argument.as_str() {
            "-c" => {
                // is category option. Fetch value
                match args.next() {
                    Some(icon) => {
                        // fetch unicode icon for given string and store in options stuct
                        // We keep Option<char> provided by get(), so not using unwrap()
                        opts.insert(
                            String::from("icon"),
                            icons.get(icon.as_str()).unwrap().to_string(),
                        );
                    }
                    None => println!("Warnng, no argument for category found. Ignoring it."),
                }
            }
            "-m" => {
                // is imessage option. Fetch value
                match args.next() {
                    Some(msg) => {
                        opts.insert(String::from("msg"), msg);
                    }
                    None => println!("Warning, no argument for message found. Ignoring it."),
                }
            }
            "-n" => {
                opts.insert(String::from("nosend"), String::from("true"));
            }
            "-v" => {
                opts.insert(String::from("verbose"), String::from("true"));
            }
            "-f" => match args.next() {
                Some(cfgfile) => {
                    opts.insert(String::from("cfgfile"), String::from(cfgfile.as_str()));
                }
                None => println!("Warning, no filename for '-f' option found. Using default."),
            },
            "-h" => {
                show_help();
            }
            x => {
                println!("Warning, ignoring unknown option:  {x}");
                show_help();
            }
        }
    }
}
