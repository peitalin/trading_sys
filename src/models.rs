use crate::currency_pairs::CurrencyPair;
use crate::schema::posts;
use crate::schema::trades;
use crate::serde_parsers::{
    deserialize_as_f32, deserialize_as_f64, deserialize_as_naive_date_time,
};
use chrono::NaiveDateTime;

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

#[derive(FromSqlRow)]
pub struct TradeDataQuery {
    pub trade_id: i32,             // Trade ID
    pub event: String,             // Event type
    pub event_time: NaiveDateTime, // Event time
    pub symbol: CurrencyPair,      // Symbol
    pub price: f32,                // Price
    pub quantity: f32,             // Quantity
    pub buyer_order_id: i32,       // Buyer order ID
    pub seller_order_id: i32,      // Seller order ID
    pub buyer_mkt_maker: bool,     //  is buyer the market maker?
}

use diesel::deserialize::Queryable;

impl Queryable<trades::SqlType, diesel::pg::Pg> for TradeDataQuery {
    type Row = (
        i32,
        String,
        NaiveDateTime,
        CurrencyPair,
        f32,
        f32,
        i32,
        i32,
        bool,
    );

    fn build(row: Self::Row) -> Self {
        TradeDataQuery {
            trade_id: row.0,        // Trade ID
            event: row.1,           // Event type
            event_time: row.2,      // Event time
            symbol: row.3,          // Symbol
            price: row.4,           // Price
            quantity: row.5,        // Quantity
            buyer_order_id: row.6,  // Buyer order ID
            seller_order_id: row.7, // Seller order ID
            buyer_mkt_maker: row.8, //  is buyer the market maker?
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "trades"]
pub struct TradeDataInsert<'a> {
    #[serde(rename = "t")]
    pub trade_id: i32, // Trade ID
    #[serde(rename = "e")]
    pub event: &'a str, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: String, // Symbol
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "p")]
    pub price: f32, // Price
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "q")]
    pub quantity: f32, // Quantity
    #[serde(rename = "b")]
    pub buyer_order_id: i32, // Buyer order ID
    #[serde(rename = "a")]
    pub seller_order_id: i32, // Seller order ID
    #[serde(rename = "m")]
    pub buyer_mkt_maker: bool, //  is buyer the market maker?
}

impl<'a> std::fmt::Display for TradeDataInsert<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}
