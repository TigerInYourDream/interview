mod prepare;
mod twitter;
mod model;
mod btc;

use serde::Deserialize;
use serde::Serialize;
use tokio::time;
use crate::twitter::{get_id, get_twitter, is_new};
use crate::btc::btc_task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (t_client, b_client) = prepare::prepare();
    let user_name = get_id(&t_client).await;
    let id = user_name.unwrap().data.id;

    let mut interval = time::interval(time::Duration::from_secs(10));
    loop {
        interval.tick().await;
        let res = get_twitter(&t_client, id.as_str()).await;
        let need_btc = is_new(res).await;
        if need_btc {
            tracing::warn!("Warning: Musk have send new tweet!!!!!!!!!!!");
            btc_task(&b_client).await;
        }
    }
}