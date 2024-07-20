use plotters::prelude::*;
use chrono::{DateTime, FixedOffset};
use crate::backtester::Backtest;
use plotters::coord::types::RangedCoordf64;

//pub fn plot(quotes:&Data, position:&Vec<f64>, account:&Vec<f64>, orders:&Vec<Order>) ->Result<(), Box<dyn std::error::Error>>{
pub fn plot_old(backtest:Backtest) ->Result<(), Box<dyn std::error::Error>>{
    let yahoo_datetimes:Vec<DateTime<FixedOffset>> = backtest.quotes().timestamps();
    let opens:Vec<f64> = backtest.quotes().open();
    let highs:Vec<f64> = backtest.quotes().high();
    let lows:Vec<f64> = backtest.quotes().low();
    let closes:Vec<f64> = backtest.quotes().close();

    // if folder does not exists no plotting happens: should implement check and create folder
    let root = BitMapBackend::new("examples/images/plot.png", (1024, 768)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let (upper,lower) = root.split_vertically(512);

    let min_lows = *lows.iter().min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()-5.0;
    let max_highs = *highs.iter().max_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()+5.0;

    let mut chart = ChartBuilder::on(&upper)
        .margin(5)
        .caption("Chart ".to_owned() + backtest.quotes().ticker(), ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len()-1], min_lows..max_highs).unwrap();

    chart.configure_mesh().x_label_formatter(&|dt|dt.format("%Y-%m-%d").to_string()).draw().unwrap();

    let Some(indicator) = backtest.strategy().indicator() else { todo!() };
    for nr in indicator.iter(){
        let Some(index) = nr.iter().position(|&x|x!=-1.0) else {todo!()};
        let _ = chart.draw_series(LineSeries::new((index..closes.len()).map(|i|(yahoo_datetimes[i],nr[i])),&GREEN)).unwrap().label("indicator");
    }


    struct CustomRow {//no need to have a dedicated struct?
        date: DateTime<FixedOffset>,
        value1: f64,
        value2: f64,
        value3: f64,
        value4: f64,
    }
    let x:Vec<CustomRow> = yahoo_datetimes.iter().zip(opens.iter()).zip(highs.iter()).zip(lows.iter()).zip(closes.iter())
        .map(|((((date,&open),&high),&low),&close)|CustomRow{date:*date,value1:open,value2:high,value3:low,value4:close }).collect();

    let _ = chart.draw_series(
        x.iter().map(|x| {
            CandleStick::new(x.date,x.value1, x.value2, x.value3, x.value4, GREEN.filled(), RED, 15)
        }),);

    //add marker and label
    for ((x, y),z) in yahoo_datetimes.iter().zip(closes.iter()).zip(backtest.orders().iter()) {
        chart.draw_series(PointSeries::of_element(
            vec![(*x, *y)],
            5, // Circle marker size
            &RED, // Red color
            &|c, s, st| {
                // Customize the circle marker appearance if needed
                return EmptyElement::at(c) + Circle::new((0, 10), s, st.filled()) +
                    Text::new(format!("{:?}", z), (0, 15), ("sans-serif", 15)) +
                    TriangleMarker::new((-4, -4), 4, RED);
            },
        ))?;
    }

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();


    let networth:Vec<f64> = closes.iter().zip(backtest.position().iter()).zip(backtest.account().iter()).map(|((&a,&b),&c)|a*b+c).collect();
    let min_nw = *networth.iter().min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()-5000.0;
    let max_nw = *networth.iter().max_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()+5000.0;

    let mut chart_low = ChartBuilder::on(&lower)
        .margin(5)
        .caption("Net worth", ("sans-serif", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len()-1], min_nw..max_nw).unwrap();

    chart_low.configure_mesh().x_label_formatter(&|dt|dt.format("%Y-%m-%d").to_string()).draw().unwrap();


    let _ = chart_low.draw_series(LineSeries::new((0..networth.len()).map(|i| (yahoo_datetimes[i], networth[i])), &BLUE)).unwrap().label("networth");

    chart_low.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
    print!("Chart saved as .../examples/images/plot.png");
    Ok(())
}

pub fn plot(backtest:Backtest, config:Plot_Config) ->Result<(), Box<dyn std::error::Error>>{
    ///function used to plot data, indicators and equity
    /// modify Plot_Config to define different chart parameters or apply default
    //let indic_b = indic.unwrap_or(true);
    let yahoo_datetimes:Vec<DateTime<FixedOffset>> = backtest.quotes().timestamps();
    let opens:Vec<f64> = backtest.quotes().open();
    let highs:Vec<f64> = backtest.quotes().high();
    let lows:Vec<f64> = backtest.quotes().low();
    let closes:Vec<f64> = backtest.quotes().close();

    // if folder does not exists no plotting happens: should implement check and create folder
    let root = BitMapBackend::new("examples/images/plot.png", (1024, 768)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let (upper,lower) = root.split_vertically(512);

    let min_lows = *lows.iter().min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()-5.0;
    let max_highs = *highs.iter().max_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()+5.0;

    let mut chart: ChartContext<BitMapBackend, Cartesian2d<RangedDateTime<DateTime<FixedOffset>>, RangedCoordf64>>;

    if config.display_networth == true {
        chart = ChartBuilder::on(&upper)
            .margin(5)
            .caption("Chart ".to_owned() + backtest.quotes().ticker(), ("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len() - 1], min_lows..max_highs).unwrap();
    } else {
        chart = ChartBuilder::on(&root)
            .margin(5)
            .caption("Chart ".to_owned() + backtest.quotes().ticker(), ("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len() - 1], min_lows..max_highs).unwrap();
    };

    chart.configure_mesh().x_label_formatter(&|dt|dt.format("%Y-%m-%d").to_string()).draw().unwrap();

    if config.display_indic ==true {
        let Some(indicator) = backtest.strategy().indicator() else { todo!() };
        for nr in indicator.iter() {
            let Some(index) = nr.iter().position(|&x| x != -1.0) else { todo!() };
            let _ = chart.draw_series(LineSeries::new((index..closes.len()).map(|i| (yahoo_datetimes[i], nr[i])), &GREEN)).unwrap().label("indicator");
        }
    }


    struct CustomRow {//no need to have a dedicated struct?
        date: DateTime<FixedOffset>,
        value1: f64,
        value2: f64,
        value3: f64,
        value4: f64,
    }
    let x:Vec<CustomRow> = yahoo_datetimes.iter().zip(opens.iter()).zip(highs.iter()).zip(lows.iter()).zip(closes.iter())
        .map(|((((date,&open),&high),&low),&close)|CustomRow{date:*date,value1:open,value2:high,value3:low,value4:close }).collect();

    let _ = chart.draw_series(
        x.iter().map(|x| {
            CandleStick::new(x.date,x.value1, x.value2, x.value3, x.value4, GREEN.filled(), RED, 15)
        }),);

    //add marker and label
    for ((x, y),z) in yahoo_datetimes.iter().zip(closes.iter()).zip(backtest.orders().iter()) {
        chart.draw_series(PointSeries::of_element(
            vec![(*x, *y)],
            5, // Circle marker size
            &RED, // Red color
            &|c, s, st| {
                // Customize the circle marker appearance if needed
                return EmptyElement::at(c) + Circle::new((0, 10), s, st.filled()) +
                    Text::new(format!("{:?}", z), (0, 15), ("sans-serif", 15)) +
                    TriangleMarker::new((-4, -4), 4, RED);
            },
        ))?;
    }

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    if config.display_networth {
        let networth: Vec<f64> = closes.iter().zip(backtest.position().iter()).zip(backtest.account().iter()).map(|((&a, &b), &c)| a * b + c).collect();
        let min_nw = *networth.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap() - 5000.0;
        let max_nw = *networth.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap() + 5000.0;

        let mut chart_low = ChartBuilder::on(&lower)
            .margin(5)
            .caption("Net worth", ("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(yahoo_datetimes[0]..yahoo_datetimes[yahoo_datetimes.len() - 1], min_nw..max_nw).unwrap();

        chart_low.configure_mesh().x_label_formatter(&|dt| dt.format("%Y-%m-%d").to_string()).draw().unwrap();

        let _ = chart_low.draw_series(LineSeries::new((0..networth.len()).map(|i| (yahoo_datetimes[i], networth[i])), &BLUE)).unwrap().label("networth");

        chart_low.configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .draw()
            .unwrap();
    }
    print!("Chart saved as .../examples/images/plot.png");
    Ok(())
}

pub struct Plot_Config{
    display_indic: bool,
    display_networth: bool,
}

impl Default for Plot_Config{
    fn default() -> Self {
        Self{
            display_indic:false,
            display_networth:false,
        }
    }
}