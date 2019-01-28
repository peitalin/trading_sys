

extern crate chrono;
extern crate regex;

extern crate reqwest;
extern crate clap;
extern crate scraper;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod coinmarketcap;
mod nomics;


fn main() -> std::io::Result<()> {

    Ok(nomics::nomics_main())

    // Ok(coinmarketcap::cmc_main())

}






