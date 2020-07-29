///
/// TODO why we need this line `#[]`?
///
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

/// how tcp listener work? what if udp?
use std::net::TcpListener;

///derive?
#[derive(Deserialize, Debug)]
struct Server {
    name: Option<String>,
    port: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Conf {
    server: Option<Vec<Server>>,
}

fn main() {
    let config_filepath = "./server.toml";
    let mut file = match File::open(config_filepath) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", config_filepath, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    let config: Conf = toml::from_str(&str_val).unwrap();

    for server in config.server.unwrap() {
        let ip = String::from("127.0.0.1");
        let port = String::from(server.port.unwrap());
        let listener = TcpListener::bind(ip + &port).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
        }
    }
}
