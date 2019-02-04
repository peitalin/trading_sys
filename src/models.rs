

use chrono::NaiveDateTime;
use crate::schema::posts;
use crate::schema::trade;
use crate::serde_parsers::{
    deserialize_as_naive_date_time,
    deserialize_as_f64,
    deserialize_as_f32
};
use crate::currency_pairs::{ CurrencyPair };


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Debug, Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[table_name="trade"]
pub struct TradeData {
    #[serde(rename = "e")]
    pub event: String,             // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with="deserialize_as_naive_date_time")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair,      // Symbol
    #[serde(rename = "t")]
    pub trade_id: i32,             // Trade ID
    #[serde(deserialize_with="deserialize_as_f32")]
    #[serde(rename = "p")]
    pub price: f32,                // Price
    #[serde(deserialize_with="deserialize_as_f32")]
    #[serde(rename = "q")]
    pub quantity: f32,             // Quantity
    #[serde(rename = "b")]
    pub buyer_order_id: i32,       // Buyer order ID
    #[serde(rename = "a")]
    pub seller_order_id: i32,      // Seller order ID
    #[serde(rename = "m")]
    pub buyer_mkt_maker: bool,     //  is buyer the market maker?
}

impl std::fmt::Display for TradeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}

