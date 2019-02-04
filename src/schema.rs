table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    trades (trade_id) {
        trade_id -> Int4,
        event -> Text,
        event_time -> Timestamp,
        symbol -> Text,
        price -> Float4,
        quantity -> Float4,
        buyer_order_id -> Int4,
        seller_order_id -> Int4,
        buyer_mkt_maker -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(posts, trades,);
