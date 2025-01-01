use csv::Writer;
use crate::datas::Data;
use crate::orders::Order;
use crate::orders::Order::{BUY,SHORTSELL,NULL};
use std::error::Error;
use crate::ta::{Indicator,sma,rsi};

/// Struct to hold vector of choices and indicators.
/// There is no specific constructor.
/// Need to be created via a user-defined function which return a Strategy
#[derive(Clone)]
pub struct Strategy{
    pub name:String,
    pub choices:Vec<Order>,
    pub indicator:Option<Vec<Vec<f64>>>,
}

impl Strategy{
    pub fn choices(&self)->Vec<Order>{
        return self.choices.clone();
    }
    pub fn name(&self)->&String{ return &self.name;}
    pub fn indicator(&self)->Option<Vec<Vec<f64>>>{ return self.indicator.clone();}
    pub fn invert(&self) ->Self{
        let length = self.choices.len();
        let mut inv_choices = self.choices.clone();
        for i in 0..length{
            if self.choices[i]==BUY { inv_choices[i]=SHORTSELL} else if self.choices[i]==SHORTSELL { inv_choices[i]=BUY}
        }
        let indicator = self.indicator.clone();
        Strategy{
            name:"invert".to_string(),
            choices: inv_choices,
            indicator,
        }
    }
    pub fn to_csv(&self, filename:&str)->Result<(),Box<dyn Error>>{
        let mut wrt = Writer::from_path(filename)?;
        let choices_transpose:Vec<Vec<String>>= self.choices.iter().map(|e|vec![e.clone().to_string().to_string()]).collect();
        wrt.serialize("choices")?;
        for col in choices_transpose.iter(){
        wrt.serialize(col)?;}
        wrt.flush()?;
        Ok(())
    }
}

pub fn buy_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![BUY;length];
    let name = "buy and hold".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn short_n_hold(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![SHORTSELL;length];
    let name = "short and hold".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn do_nothing(quotes:Data)->Strategy{
    let length = quotes.timestamps().len();
    let choices = vec![NULL;length];
    let name = "do nothing".to_string();
    let indicator = Some(vec![vec![-1.;length]]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn simple_sma(quotes:Data, period:usize) ->Strategy{
    let sma = sma(&quotes,period);
    let indicator = Indicator{indicator:sma,quotes:quotes};
    let length = indicator.quotes.timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 0..length{
        if indicator.indicator[i]!=-1.{
            if indicator.indicator[i]>=indicator.quotes.open()[i]{
                choices[i] = BUY;
            }else if indicator.indicator[i]<indicator.quotes.open()[i]{
               choices[i] = SHORTSELL}
        }
    }
    let name = format!("simple_sma_{}",period);
    let indicator = Some(vec![indicator.indicator()]);
    Strategy{
        name:name,
        choices:choices,
        indicator,
    }
}
pub fn sma_cross(quotes:Data, short_period:usize, long_period:usize)->Strategy{
    let sma_short = sma(&quotes, short_period);
    let sma_long = sma(&quotes, long_period);
    let ind_short = Indicator{indicator:sma_short,quotes:quotes.clone()};
    let ind_long = Indicator{indicator:sma_long, quotes:quotes.clone()};
    let length = ind_short.quotes().timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 0..length{
        if ind_long.indicator()[i]!=-1.{
            if ind_short.indicator()[i]>ind_long.indicator()[i]{choices[i]=BUY}
            else {choices[i]=SHORTSELL};
        }
    }
    let name=format!("sma_cross_{}_{}",short_period,long_period);
    let indicator = Some(vec![ind_short.indicator(),ind_long.indicator()]);
    Strategy{
        name:name,
        choices:choices,
        indicator:indicator,
    }
}
pub fn rsi_strategy(quotes:Data, period:usize)->Strategy{
    let rsi = rsi(&quotes,period);
    let indicator = Indicator{indicator:rsi,quotes};
    let length = indicator.quotes().timestamps().len();
    let mut choices = vec![NULL;length];
    for i in 0..length{
        if indicator.indicator()[i]!=-1.{
            if indicator.indicator()[i]>70.{choices[i]=SHORTSELL}
            else if indicator.indicator()[i]<30. {choices[i]=BUY}
        }
    }
    let name = format!("rsi_{}",period);
    let indicator=Some(vec![indicator.indicator()]);
    Strategy{
        name,
        choices,
        indicator,
    }
}