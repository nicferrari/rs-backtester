use backtester::Result;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::reports::print_report;
use backtester::strategies::buy_n_hold;

#[test]
fn init_backtester()->Result<()>{
    let quotes = Data::new_from_yahoo("AAPL".to_string())?;
    let strategy = buy_n_hold(quotes.clone());
    let tester = Backtest::new(quotes.clone(),strategy.clone(),100000f64)?;
    Ok(())
}