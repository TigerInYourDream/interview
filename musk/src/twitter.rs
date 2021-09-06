use reqwest::Client;
use tracing::{info};
use crate::model::{UserName, TweetTimeline, Tweet};
use crate::prepare::init_config;
use std::error::Error;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::fs::OpenOptions;
use chrono::{DateTime, Utc};

pub async fn get_id(client: &Client) -> reqwest::Result<UserName> {
    let map = init_config();
    let who = map.get("who").ok_or("failed get username form config").unwrap();
    let who = who.to_owned().into_str().unwrap();
    info!("Will get id form {}",&who);
    let url = format!("https://api.twitter.com/2/users/by/username/{}", who);
    let resp = client.get(url).send()
                     .await?;
    resp.json::<UserName>().await
}

pub async fn get_twitter(client: &Client, id: &str) -> TweetTimeline {
    let url = format!("https://api.twitter.com/2/users/{}/tweets?tweet.fields=created_at&max_results=5&exclude=retweets,replies", id);
    let resp = client.get(url).send().await.unwrap();
    let res = resp.json::<TweetTimeline>().await.unwrap();
    res
}

pub async fn is_new(timeline: TweetTimeline) -> bool {
    let newest_id = timeline.meta.newest_id;
    let tweet = timeline.data.iter().find(|&t| t.id == newest_id);
    if tweet.is_none() {
        return false;
    }
    let tweet = tweet.to_owned();
    info!("{:?}", &tweet.unwrap());

    let record = read_new_file().await;
    match record {
        None => {
            write_file_append(tweet.unwrap()).await.unwrap();
            write_file_new(tweet.unwrap()).await.unwrap();
            return true;
        }
        Some(record) => {
            let record = record.created_at;
            let time2 = &tweet.unwrap().created_at;
            let record = record.parse::<DateTime<Utc>>().unwrap();
            let time2 = time2.parse::<DateTime<Utc>>().unwrap();

            if time2.timestamp() > record.timestamp() {
                write_file_append(tweet.unwrap()).await.unwrap();
                write_file_new(tweet.unwrap()).await.unwrap();
                return true;
            } else {
                return false;
            }
        }
    }
}

pub async fn write_file_append(tweet: &Tweet) -> Result<(), Box<dyn Error>> {
    let mut options = OpenOptions::new();
    let mut file = options.append(true).write(true).open("data/data.txt").await?;
    file.write_all(serde_json::to_string(tweet).unwrap().as_bytes()).await?;
    file.write_all("\n".as_bytes()).await?;
    Ok(())
}

pub async fn write_file_new(tweet: &Tweet) -> Result<(), Box<dyn Error>> {
    let mut options = OpenOptions::new();
    let mut file = options.append(false).write(true).open("data/new.txt").await?;
    file.write_all(serde_json::to_string(tweet).unwrap().as_bytes()).await?;
    file.write_all("\n".as_bytes()).await?;
    Ok(())
}

pub async fn read_new_file() -> Option<Tweet> {
    let mut options = OpenOptions::new();
    let mut file = options.read(true).open("data/new.txt").await.unwrap();
    let mut res = String::new();
    file.read_to_string(&mut res).await.unwrap();
    let tweet = serde_json::from_str::<Tweet>(res.as_str());
    match tweet {
        Ok(tweet) => { Some(tweet) }
        Err(_) => { None }
    }
}