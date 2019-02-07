
/////////////////////////////////////////////////////////////////
/// Get data from databases
/////////////////////////////////////////////////////////////////

pub fn get_klines_from_postgres() {
    use diesel::prelude::*;
    use trading_sys::models::klines::KlineData;
    use trading_sys::schema::klines::dsl::*; // .get_result trait

    let connection = trading_sys::establish_connection_pg();

    println!("{:?}", klines);
    let results: Vec<KlineData> = klines
        .limit(6)
        .load::<KlineData>(&connection)
        .expect("Error in deserializing Jsonb to Klines.");

    for result in results {
        println!("\n{}", result.symbol);
        println!(
            "open: ${}\nclose: ${}\nhigh: ${}\nlow: ${}\n",
            result.open,
            result.close,
            result.high,
            result.low,
        );
        println!("-------\n");
    }
}

pub fn get_book_depth_from_postgres() {
    use diesel::prelude::*;
    use trading_sys::models::book_depth::BookDepthData;
    use trading_sys::schema::book_depth::dsl::*; // .get_result trait

    let connection = trading_sys::establish_connection_pg();

    println!("{:?}", book_depth);
    let results = book_depth
        .limit(6)
        .load::<BookDepthData>(&connection)
        .expect("Error in deserializing Jsonb to BookDepthDataQuery.");

    for trade_result in results {
        println!("\n{}", trade_result.event_time);
        println!(
            "${}: {:?} {:?}",
            trade_result.symbol.to_uppercase(),
            trade_result.asks,
            trade_result.bids
        );
        println!("-------\n");
    }
}

pub fn get_trades_from_postgres() {
    use diesel::prelude::*;
    use trading_sys::models::trades::TradeData;
    use trading_sys::schema::trades::dsl::*; // .get_result trait

    let connection = trading_sys::establish_connection_pg();

    let results = trades
        .filter(quantity.gt(1.0))
        .limit(5)
        .load::<TradeData>(&connection)
        .expect("Error loading posts");

    println!(
        "Displaying {} trades, each greater than 1.0 ETH",
        results.len()
    );

    for trade_result in results {
        println!("\n{}", trade_result.event_time);
        println!(
            "${}: {} {}",
            trade_result.price,
            trade_result.quantity,
            trade_result.symbol.to_uppercase()
        );
        println!("-------\n");
    }
}

pub fn get_aggregate_trades_from_postgres() {
    use diesel::prelude::*;
    use trading_sys::models::aggregate_trades::AggregateTradeData;
    use trading_sys::schema::aggregate_trades::dsl::*; // .get_result trait

    let connection = trading_sys::establish_connection_pg();

    let results: Vec<AggregateTradeData> = aggregate_trades
        .load::<AggregateTradeData>(&connection)
        .expect("Error in deserializing AggregateTradeData.");

    for result in results {
        println!("\n{}", result);
        println!("-------\n");
    }
}

// pub fn get_mini_ticker_from_postgres() {
//     use diesel::prelude::*;
//     use trading_sys::models::mini_ticker::MiniTickerData;
//     use trading_sys::schema::mini_ticker::dsl::*; // .get_result trait
//
//     let connection = trading_sys::establish_connection_pg();
//
//     let results: Vec<MiniTickerData> = MiniTickerData
//         .load::<MiniTickerData>(&connection)
//         .expect("Error in deserializing AggregateTradeData.");
//
//     for result in results {
//         println!("\n{}", result);
//         println!("-------\n");
//     }
// }
