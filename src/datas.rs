use chrono::{DateTime, FixedOffset, TimeZone};
use csv::Writer;
//use serde::de::Error;
//use std::error::Error;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::Quote;
use tokio_test;
use crate::errors::Result;

pub fn download_data(ticker:&str, interval:&str, range:&str)->Result<Vec<Quote>>{
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, interval, range)).unwrap();
    let quotes = response.quotes().unwrap();
    return Ok(quotes);
}
///struct to contain all market data (ticker + OHLC)
#[derive(Clone)]
pub struct Data{
    ticker:String,
    datetime:Vec<DateTime<FixedOffset>>,
    open:Vec<f64>,
    high:Vec<f64>,
    low:Vec<f64>,
    close:Vec<f64>,
}

impl Data{
    pub fn new_from_yahoo(ticker:String) ->Result<Self>{
        let quotes = download_data(&ticker,"1d","1mo")?;
        let timestamps:Vec<u64> = quotes.iter().map(|s|s.timestamp).collect();
        let yahoo_datetimes: Vec<DateTime<FixedOffset>> = timestamps.iter().map(|&ts|{FixedOffset::east_opt(0).unwrap().timestamp_opt(ts as i64,0).unwrap()}).collect();
        let opens:Vec<f64> = quotes.iter().map(|s|s.open).collect();
        let highs:Vec<f64> = quotes.iter().map(|s|s.high).collect();
        let lows:Vec<f64> = quotes.iter().map(|s|s.low).collect();
        let closes:Vec<f64> = quotes.iter().map(|s|s.close).collect();
        Ok(Data{
            ticker:ticker,
            datetime:yahoo_datetimes,
            open:opens,
            high:highs,
            low:lows,
            close:closes,
        })
    }
    pub fn save(&self)->Result<()>{
        let mut wrt = Writer::from_path("savedata.csv").expect("path not valid");
        let dates_t:Vec<Vec<String>> = self.datetime.iter().map(|e|vec![e.to_string()]).collect();
        let open_t:Vec<Vec<String>> = self.open.iter().map(|e|vec![e.to_string()]).collect();
        let high_t:Vec<Vec<String>> = self.high.iter().map(|e|vec![e.to_string()]).collect();
        let low_t:Vec<Vec<String>> = self.low.iter().map(|e|vec![e.to_string()]).collect();
        let close_t:Vec<Vec<String>> = self.close.iter().map(|e|vec![e.to_string()]).collect();
        wrt.serialize(("DATE","OPEN","HIGH","LOW","CLOSE")).expect("cannot write data");
        for ((((date,open),high),low),close) in dates_t.iter().zip(open_t.iter()).zip(high_t.iter()).zip(low_t.iter()).zip(close_t.iter()){
            wrt.serialize((date,open,high,low,close)).expect("cannot write data");
        }
        wrt.flush().expect("cannot write file");
        Ok(())
    }
    pub fn ticker(&self)->&str{
        return &*self.ticker;
    }
    pub fn timestamps(&self)->Vec<DateTime<FixedOffset>>{
        return self.datetime.clone();
    }
    pub fn open(&self)->Vec<f64>{
        return self.open.clone();
    }
    pub fn high(&self)->Vec<f64>{return self.high.clone();}
    pub fn low(&self)->Vec<f64>{return self.low.clone();}
    pub fn close(&self)->Vec<f64>{
        return self.close.clone();
    }
}