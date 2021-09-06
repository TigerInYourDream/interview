use tracing_subscriber::FmtSubscriber;
use tracing::Level;
use config::{Config, Source, Value};
use std::collections::HashMap;
use config::File;
use tracing::{error, info};
use reqwest::{header, Client};
use reqwest::header::HeaderMap;
use std::fs;

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

pub fn init_data_file() {
    let file = fs::File::open("data");
    match file {
        Ok(_file) => {
            ()
        }
        Err(_) => {
            fs::create_dir_all("data/").unwrap();
            fs::File::create("data/data.txt").unwrap();
            fs::File::create("data/new.txt").unwrap();
        }
    }
}

pub fn init_config() -> HashMap<String, Value> {
    let mut settings = Config::default();
    let merge = settings.merge(File::with_name("config/config.toml"));
    if merge.is_err() {
        error!("init config failed, please check config/config.toml in your project directory");
        panic!();
    }
    let config = merge.expect("init config failed, please check data config/config.toml");
    config.collect().unwrap()
}

fn init_twitter_client(map: HashMap<String, Value>) -> Client {
    let bearer = map.get("Bearer");
    let who = map.get("who");
    match who {
        None => {
            panic!("cannot find who in config/config.toml, please add it")
        }
        Some(who) => {
            let who = who.to_owned();
            let who = who.into_str().unwrap();
            info!("will get twitter form {}  ......",who);
        }
    }

    match bearer {
        None => {
            panic!("cannot find Bearer in config/config.toml, please add it")
        }
        Some(bearer) => {
            let bearer = bearer.to_owned();
            let bearer = bearer.into_str().unwrap();
            let mut headers = HeaderMap::new();
            headers.insert(header::AUTHORIZATION, bearer.parse().unwrap());
            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap();
            return client;
        }
    }
}

pub fn init_btc_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip"));
    let client = reqwest::Client::builder()
        .build()
        .unwrap();
    return client;
}


pub fn prepare() -> (Client, Client) {
    init_tracing();
    init_data_file();
    let map = init_config();
    (init_twitter_client(map), init_btc_client())
}