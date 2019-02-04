table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    trade (trade_id) {
        trade_id -> Int4,
        event -> Nullable<Text>,
        event_time -> Nullable<Timestamp>,
        symbol -> Nullable<Text>,
        price -> Nullable<Float4>,
        quantity -> Nullable<Float4>,
        buyer_order_id -> Nullable<Int4>,
        seller_order_id -> Nullable<Int4>,
        buyer_mkt_maker -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    trade,
);
