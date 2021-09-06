use reqwest::Client;
use tracing::info;
use tokio::time;
use serde_json::Value;
use crate::model::BtcPrice;

pub async fn get_btc(client: &Client) {
    let url = format!("https://api.coincap.io/v2/assets/bitcoin");
    let resp = client.get(url).send()
                     .await.unwrap();
    //let r = resp.json::<Value>().await.unwrap();
    let r = resp.text().await.unwrap();
    info!("{:#?}",r);
}

pub async fn btc_task(client: &Client) {
    let mut interval = time::interval(time::Duration::from_secs(30));
    for _i in 0..5 {
        interval.tick().await;
        get_btc(client).await;
    }
}