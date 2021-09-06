mod file;
mod trade;

use std::error::Error;
use crate::file::{read_file};
use crate::trade::sale;


fn main() -> Result<(), Box<dyn Error>> {
    let mut mont = 0f64;
    let sort_file = read_file()?;
    for (index, _time) in sort_file.sort.iter().enumerate() {
        if index > sort_file.sort.len() - 2 {
            break;
        }
        if index % 2 == 0 {
            let buy_path = sort_file.map.get(&sort_file.sort[index]).ok_or("get buy_path error")?;
            let sale_path = sort_file.map.get(&sort_file.sort[index + 1]).ok_or("get sale_path error")?;
            mont += sale(sale_path, buy_path);
            println!("{:?} {:?}", buy_path, sale_path);
        }
    }
    println!("--------------------------------------------------------------------------------------");
    println!("at end yuo will got {} usdt", mont);
    Ok(())
}