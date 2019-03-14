
## Binance API and trading system in Rust

Preliminary side project.
Only Websocket APIs are currently available. Deposits + orders + trade execution is under development.

1. Setup PostgreSQL database with diesel.
```
$ export DATABASE_URL=postgres://{username}:{password}@localhost/trading_sys
$ diesel migration run
```

2. Start Binance websockets API.
```
cargo run --bin binance
```

3. Entry point is `src/bin/binance/main.rs`, edit to explore other functions.


4. Coinmarketcap API
```
cargo run --bin coinmarketcap -- --currency monero --start-date 2017-01-01 --end-date 2019-01-01
```


