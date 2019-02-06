use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};
use std::fmt;

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time};

use crate::schema::posts;

pub mod trades;
pub mod aggregate_trades;
pub mod book_depth;
pub mod kline;


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}






///////////////////////////////////////////////////////////////////////////////
// #[derive(FromSqlRow)]
// pub struct TradeDataQuery {
//     pub trade_id: i32,             // Trade ID
//     pub event: String,             // Event type
//     pub event_time: NaiveDateTime, // Event time
//     pub symbol: CurrencyPair,      // Symbol
//     pub price: f32,                // Price
//     pub quantity: f32,             // Quantity
//     pub buyer_order_id: i32,       // Buyer order ID
//     pub seller_order_id: i32,      // Seller order ID
//     pub buyer_mkt_maker: bool,     //  is buyer the market maker?
// }
//
// use diesel::deserialize::Queryable;
//
// impl Queryable<trades::SqlType, diesel::pg::Pg> for TradeData {
//     type Row = (
//         i32,
//         String,
//         NaiveDateTime,
//         CurrencyPair,
//         f32,
//         f32,
//         i32,
//         i32,
//         bool,
//     );
//
//     fn build(row: Self::Row) -> Self {
//         TradeData {
//             trade_id: row.0,        // Trade ID
//             event: row.1,           // Event type
//             event_time: row.2,      // Event time
//             symbol: row.3,          // Symbol
//             price: row.4,           // Price
//             quantity: row.5,        // Quantity
//             buyer_order_id: row.6,  // Buyer order ID
//             seller_order_id: row.7, // Seller order ID
//             buyer_mkt_maker: row.8, //  is buyer the market maker?
//         }
//     }
// }
