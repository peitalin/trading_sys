

use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::TradeData;


pub fn create_trade<'a> (conn: &PgConnection, trade_data: &TradeData) {
    use crate::schema::trade;

    let res = diesel::insert_into(trade::table)
        .values(trade_data)
        .execute(conn);

    // let res: TradeData = diesel::insert_into(trade::table)
    //     .values(&new_trade_data)
    //     .get_result(conn)
    //     .expect("Error saving new trade");

    println!("Database write result: {:?}\n", res);
}

