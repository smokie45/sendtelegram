use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parse values from a TOML cfgfile into a hashmap of strings
///
/// This function will get a hashmap of strings. A filename is taken
/// from the key "cfgfile" and opened. The expected TOML structure
/// is parsed and selected key value pairs added to the hashmap.
pub fn parse(opts: &mut HashMap<String, String>) {
    let path = Path::new(opts.get("cfgfile").expect("Error, no option 'cfgfile'."));

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {} [{why}]", &path.display()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l: &String = &line.expect("Error reading cfgfile");

        if l.starts_with("#") {
        } else if l.starts_with("API") {
            match l.find('=') {
                Some(x) => {
                    opts.insert(String::from("API"), l[(x + 1)..].trim().to_string().clone());
                }
                None => println!("Error, no '=' found while reading API"),
            }
        } else if l.starts_with("CHAT") {
            match l.find('=') {
                Some(x) => {
                    opts.insert(
                        String::from("CHAT"),
                        l[(x + 1)..].trim().to_string().clone(),
                    );
                }
                None => println!("Error, no '=' found while reading CHAT"),
            }
        }
    }
}
