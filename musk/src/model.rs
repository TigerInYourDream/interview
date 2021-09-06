use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    pub data: NameDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameDetail {
    pub id: String,
    name: String,
    username: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TweetTimeline {
    pub data: Vec<Tweet>,
    pub meta: MetaData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub id: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub newest_id: String,
    pub next_token: String,
    pub oldest_id: String,
    pub result_count: u8,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BtcPrice {
    pub data: BtcDetail,
    pub timestamp: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BtcDetail {
    pub changePercent24Hr: String,
    pub explorer: String,
    pub id: String,
    pub marketCapUsd: String,
    pub maxSupply: String,
    pub name: String,
    pub priceUsd: String,
    pub rank: u32,
    pub supply: String,
    pub symbol: String,
    pub volumeUsd24Hr: String,
    pub vwap24Hr: String,
}