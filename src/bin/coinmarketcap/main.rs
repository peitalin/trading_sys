extern crate chrono;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate trading_sys;

use regex::Regex;
use clap::{ Arg, App };
use std::sync::Arc;

pub mod coinlist;
use coinlist::build_coinlist;

pub mod request;
use request::download_data;

pub mod filesys;
use filesys::{
    check_if_filepath_exists,
    create_data_directory,
    create_filepath
};


pub fn main() {

    let matches = parse_args();
    let (_currency, _coinlist, _start_date, _end_date) = parse_options(matches);
    create_data_directory();

    // Single currency
    if _currency.len() > 0 {
        let start_date = _start_date.clone();
        let end_date   = _end_date.clone();
        let csv_data = download_data(&_currency, start_date, end_date);
        for line in csv_data {
            println!("{}", line);
        }
    }

    // Batch multiple coins
    let coinlist_metadata = build_coinlist(_coinlist.expect("No number supplied for --coinlist n"));

    let mut queue = vec![];

    for coindata in coinlist_metadata {

        let start_date = _start_date.clone();
        let end_date   = _end_date.clone();
        let (id, rank) = match (coindata.id, coindata.rank) {
            (Some(id), Some(rank)) => (id, rank as i32),
            (_, _) => continue,
        };
        let fp = create_filepath(&rank, &id);

        match check_if_filepath_exists(&fp) {
            true => continue,
            false => {
                queue.push(std::thread::spawn(move || {
                    println!("Requesting: {:?} - {:?}", &rank, &id);
                    let csv_data = download_data(&id, start_date, end_date);
                    let _ = std::fs::write(&fp, csv_data.join("\n"));
                }))
            }
        }
    };

    for job in queue {
        let _ = job.join();
        std::thread::sleep(std::time::Duration::from_millis(250));
    };

    println!("Scraping complete.");
}


pub fn parse_date<'a>(date: Option<&str>) -> String {
    let yyyy_mm_dd = Regex::new(r"[2][0][1-9][1-9]-[0-1][0-9]-[0-3][0-9]").unwrap();
    let maybe_date = match date {
        Some(d) => match yyyy_mm_dd.is_match(d) {
            true => d.replace("-", ""),
            false => panic!("Must be yyyy-mm-dd format"),
        },
        None => panic!("Must supply a date in yyyy-mm-dd format"),
    };
    return maybe_date
}

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    let matches = App::new("Coinmarketcap Scraper")
        .version("1.0")
        .author("Peita Lin")
        .about("Scrapes historical price")
        .arg(Arg::with_name("currency")
             .short("c")
             .long("currency")
             .value_name("CURRENCY")
             .help("Name of the cryptocurrency as shown on coinmarketcap, for example: bitcoin")
             .conflicts_with("coinlist")
             .takes_value(true))
        .arg(Arg::with_name("coinlist")
             .short("l")
             .long("coinlist")
             .help("scrape the top # coins on coinmarketcap. Default is 5.")
             .conflicts_with("currency")
             .takes_value(true))
        .arg(Arg::with_name("start date")
             .short("s")
             .long("start-date")
             .help("Starting date extracting historical data. For example: '2017-10-01'")
             .takes_value(true))
        .arg(Arg::with_name("end date")
             .short("e")
             .long("end-date")
             .help("Ending date for extracting historical data. For example: '2018-11-15', yyyy-mm-dd format.")
             .takes_value(true))
        .get_matches();

    return matches
}


pub fn parse_options<'a>(matches: clap::ArgMatches<'a>) -> (String, Option<i32>, Arc<String>, Arc<String>) {
    let _start_date: String = parse_date(matches.value_of("start date"));
    let _end_date: String = parse_date(matches.value_of("end date"));
    let start_date = Arc::new(_start_date); // Automatic Reference Count
    let end_date = Arc::new(_end_date);

    let currency: String = matches.value_of("currency").unwrap_or("").to_lowercase();

    let coinlist: Option<i32> = match matches.value_of("coinlist") {
        Some(n) => Some(n.parse::<i32>().unwrap()),
        None => None,
    };

    return (currency, coinlist, start_date, end_date)
}
