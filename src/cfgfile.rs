use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)] // allow debug print
pub struct CfgOpts {
    pub api: String,
    pub chat: String,
}
// - read a file to a string
// - parse the string and keyword lines and comments lines
// - return the keyword data as struct
// TODO: search default pathes (e.g. /etc)
// TODO: use default filename if none given
pub fn parse(filename: &str) -> CfgOpts {
    let mut cfg: CfgOpts = CfgOpts {
        // TODO: use Result( String, None)
        api: String::new(),
        chat: String::new(),
    };
    let path = Path::new(filename);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {} [{why}]", &path.display()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        //TODO: try to understand ok()?
        // let l: &String = &line.ok()?;
        let l: &String = &line.expect("Error reading cfgfile");

        if l.starts_with("#") {
        } else if l.starts_with("API") {
            match l.find('=') {
                Some(x) => cfg.api = l[(x + 1)..].trim().to_string().clone(),

                None => println!("Error, no '=' found while reading API"),
            }
        } else if l.starts_with("CHAT") {
            match l.find('=') {
                Some(x) => cfg.chat = l[(x + 1)..].trim().to_string().clone(),

                None => println!("Error, no '=' found while reading CHAT"),
            }
        }
    }
    cfg
}
