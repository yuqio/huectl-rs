use crate::arg::subcommand;
use crate::util;
use std::fmt;

struct Config(huelib::Config);

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("{:#?}", self.0));
        write!(f, "{}", output)
    }
}

pub fn set(arg: subcommand::SetConfig) {
    let responses = match util::get_bridge().set_config(&arg.to_modifier()) {
        Ok(v) => v,
        Err(e) => util::print_err("Failed to set config", e),
    };
    for i in responses {
        println!("{}", i);
    }
}

pub fn get(arg: subcommand::GetConfig) {
    let bridge = util::get_bridge();
    match bridge.get_config() {
        Ok(v) => {
            if arg.json {
                match serde_json::to_string_pretty(&v) {
                    Ok(v) => println!("{}", v),
                    Err(e) => util::print_err("Failed to serialize data", e),
                };
            } else {
                println!("{}", Config(v))
            }
        }
        Err(e) => util::print_err("Failed to get scene", e),
    };
}
