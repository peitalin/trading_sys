-- Your SQL goes here
CREATE TABLE klines (
	id SERIAL PRIMARY KEY,
    event TEXT NOT NULL,
    event_time TIMESTAMP NOT NULL,
    start_time TIMESTAMP NOT NULL,
    close_time TIMESTAMP NOT NULL,
    symbol TEXT NOT NULL,
    interval TEXT NOT NULL,
    first_trade_id INT NOT NULL,
    last_trade_id INT NOT NULL,
    open REAL NOT NULL,
    close REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    volume REAL NOT NULL,
    num_of_trades INT NOT NULL,
    is_kline_closed BOOL NOT NULL,
    quote_asset_vol REAL NOT NULL,
    taker_buy_base_vol REAL NOT NULL,
    taker_buy_quote_vol REAL NOT NULL
)