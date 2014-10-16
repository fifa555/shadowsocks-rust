extern crate serialize;

use serialize::Encodable;
use serialize::json;
use std::io::{File, Read, Open};

use std::to_string::ToString;
use std::fmt::{Show, Formatter, mod};

use std::option::Option;

#[deriving(Encodable, Clone)]
pub struct Config {
    pub server: String,
    pub local: String,
    pub server_port: u16,
    pub local_port: u16,
    pub password: String,
    pub method: String,
    pub timeout: Option<u64>,
    pub fast_open: bool,
}

impl Config {
    pub fn new() -> Config {
        Config{
            server: "127.0.0.1".to_string(),
            local: "127.0.0.1".to_string(),
            server_port: 8000,
            local_port: 8000,
            password: "".to_string(),
            method: "aes-128-cfb".to_string(),
            timeout: None,
            fast_open: false,
        }
    }

    fn parse_json_object(o: &json::JsonObject) -> Option<Config> {
        let mut config = Config::new();

        for (key, value) in o.iter() {
            match key.as_slice() {
                "server" => {
                    config.server = match value.as_string() {
                        Some(v) => v.to_string(),
                        None => return None,
                    };
                },
                "server_port" => {
                    config.server_port = match value.as_i64() {
                        Some(v) => v as u16,
                        None => return None,
                    };
                },
                "local_port" => {
                    config.local_port = match value.as_i64() {
                        Some(v) => v as u16,
                        None => return None,
                    };
                },
                "password" => {
                    config.password = match value.as_string() {
                        Some(v) => v.to_string(),
                        None => return None,
                    };
                },
                "method" => {
                    config.method = match value.as_string() {
                        Some(v) => v.to_string(),
                        None => return None,
                    };
                },
                "timeout" => {
                    config.timeout = match value.as_i64() {
                        Some(v) => Some(v as u64),
                        None => return None,
                    };
                },
                "fast_open" => {
                    config.fast_open = match value.as_boolean() {
                        Some(v) => v,
                        None => return None,
                    }
                },
                _ => (),
            }
        }

        Some(config)
    }

    pub fn load_from_str(s: &str) -> Option<Config> {
        let object = match json::from_str(s) {
            Ok(obj) => { obj },
            Err(e) => return None,
        };

        let json_object = match object.as_object() {
            Some(obj) => { obj },
            None => return None,
        };

        Config::parse_json_object(json_object)
    }

    pub fn load_from_file(filename: &str) -> Option<Config> {
        let mut readeropt = File::open_mode(&Path::new(filename), Open, Read);

        let mut reader = match readeropt {
            Ok(ref mut r) => r,
            Err(..) => return None,
        };

        let object = match json::from_reader(reader) {
            Ok(obj) => { obj },
            Err(..) => return None,
        };

        let json_object = match object.as_object() {
            Some(obj) => obj,
            None => return None,
        };

        Config::parse_json_object(json_object)
    }
}

impl Show for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", json::encode(self))
    }
}