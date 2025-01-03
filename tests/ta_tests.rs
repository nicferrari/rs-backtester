use rs_backtester::datas::Data;
use std::error::Error;
use rs_backtester::ta::{Indicator, rsi};
#[test]
fn indicator_tests()->Result<(), Box<dyn Error>>{
    let quotes = &Data::new_from_yahoo("AAPL","1d","1mo")?;
    let indicator = rsi(quotes,5);
    println!("{:?}",indicator);
    println!("{:?}",quotes.close());
    let a = Indicator{indicator:indicator, quotes:quotes.clone()};
    Ok(())
}