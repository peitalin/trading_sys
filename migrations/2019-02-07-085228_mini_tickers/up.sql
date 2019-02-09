-- Your SQL goes here
CREATE TABLE mini_tickers (
	id SERIAL PRIMARY KEY,
    event TEXT NOT NULL,
    event_time TIMESTAMP NOT NULL,
    symbol TEXT NOT NULL,
    open REAL NOT NULL,
    close REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    base_asset_vol REAL NOT NULL,
    quote_asset_vol REAL NOT NULL
)