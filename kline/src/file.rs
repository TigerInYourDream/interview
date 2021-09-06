use std::{env, fs};
use std::error::Error;
use chrono::{DateTime, Utc, TimeZone};
use std::path::PathBuf;
use std::collections::HashMap;

pub const PRE_FIX: &'static str = "v3_kline_";

pub struct SortFiles {
    pub sort: Vec<DateTime<Utc>>,
    pub map: HashMap<DateTime<Utc>, PathBuf>,
}

pub fn read_file() -> Result<SortFiles, Box<dyn Error>> {
    let mut map = HashMap::new();
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );
    let path = current_dir.join("data");

    let mut time_vec = vec![];
    for (_index, entry) in fs::read_dir(path)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        let time = file2time(&path);
        time_vec.push(time);
        map.insert(time, path);
    }
    let sort = sort(time_vec);
    Ok(SortFiles {
        sort,
        map,
    })
}

pub fn file2time<'a>(path: &PathBuf) -> DateTime<Utc> {
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let last: &str = file_name.split(PRE_FIX).last().unwrap();
    let time: Vec<_> = last.split("_").collect();
    let mut time = time.iter();
    let year = time.next().unwrap().parse::<i32>().unwrap();
    let month = time.next().unwrap().parse::<u32>().unwrap();
    let day = time.next().unwrap().parse::<u32>().unwrap();
    let hour = time.next().unwrap().parse::<u32>().unwrap();
    let min = time.next().unwrap().parse::<u32>().unwrap();
    let dt = Utc.ymd(year, month, day).and_hms(hour, min, 0);
    dt
}

pub fn sort(mut v: Vec<DateTime<Utc>>) -> Vec<DateTime<Utc>> {
    v.sort_by(|a, b| {
        a.cmp(&b)
    });
    v
}