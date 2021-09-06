use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Record {
    dump: String,
    shm_id: String,
    market: String,
    pre_coin: String,
    post_coin: String,
    market_time: String,
    start: String,
    high: String,
    low: String,
    end: String,
    mont: String,

}

pub fn handle_csv(path: &PathBuf) -> Vec<Record> {
    let mut file = csv::Reader::from_path(path).unwrap();

    let header = csv::StringRecord::from(vec![
        "dump", "shm_id", "market", "pre_coin", "post_coin", "market_time", "start", "high", "low", "end", "mont",
    ]);

    let mut vector = vec![];
    for r in file.records() {
        let record = r.unwrap();
        let record = record.as_slice();
        let record: Vec<_> = record.split("\t").collect();
        let record = csv::StringRecord::from(record);
        let r: Record = record.deserialize(Some(&header)).unwrap();
        vector.push(r);
    }
    vector
}

pub fn buy(path: &PathBuf) -> Vec<Record> {
    let all = handle_csv(path);
    let mut buy = vec![];
    for (_index, r) in all.into_iter().enumerate() {
        let end = r.end.parse::<f64>().unwrap();
        let start = r.start.parse::<f64>().unwrap();
        if ((end - start) % start) > (1f64 / 100f64) {
            buy.push(r);
        }
    }
    buy
}


/// 第一轮交易中需要筛选的出来的记录，到一分钟结束后的数据集中后开始筛，
/// 现在定义 同一个交易所, 同一个币种交易对 pre_coin 和 post_coin 完全一致，为在当前交易所需要卖出的那个币种
pub fn find_in_sale<'a>(r: &'a Record, vec: &'a Vec<Record>) -> Option<&'a Record> {
    for v in vec {
        if (r.market == v.market) && (r.pre_coin == v.pre_coin) && (r.post_coin == v.post_coin) {
            return Some(v);
        }
    }
    None
}

pub fn sale(sale_path: &PathBuf, buy_path: &PathBuf) -> f64 {
    let sale = handle_csv(sale_path);
    let mut buy = buy(buy_path);
    let mut mont = 0f64;
    while let Some(buy) = buy.pop() {
        let sale_record = find_in_sale(&buy, &sale);
        if let Some(r) = sale_record {
            let end = r.end.parse::<f64>().unwrap();
            let start = r.start.parse::<f64>().unwrap();
            mont += end - start
        }
    }
    mont
}