-- Your SQL goes here
CREATE TABLE trades (
  trade_id SERIAL PRIMARY KEY,
  event TEXT NOT NULL,
  event_time TIMESTAMP NOT NULL,
  symbol TEXT NOT NULL,
  price REAL NOT NULL,
  quantity REAL NOT NULL,
  trade_time TIMESTAMP NOT NULL,
  buyer_order_id INT NOT NULL,
  seller_order_id INT NOT NULL,
  buyer_mkt_maker BOOLEAN NOT NULL
)
