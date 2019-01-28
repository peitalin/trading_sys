
use regex::Regex;


pub fn extract_data_regex(html: &str) {
    //! Extract the price history from the HTML.
    //! The CoinMarketCap historical data page has just one HTML table.
    //! This table contains the data we want.
    //! It's got one header row with the column names.
    //! We need to derive the "average" price for the provided data.

    let head_re = Regex::new(r#"(<thead>)[\s\S]*(</thead>)"#).unwrap();
    let head = match head_re.find(html) {
        Some(t) => t.as_str(),
        None => panic!("Nothing found"),
    };

    let titles_re = Regex::new(r#"(<th .*>)(.*)(</th>)"#).unwrap();
    let titles: Vec<_> = titles_re.captures_iter(head).map(|m| {
        let t = match m.get(2) {
            Some(_m) => _m.as_str(),
            None => "NA",
        };
        t.replace("*", "")
    }).collect::<Vec<_>>();



    let body_re = Regex::new(r#"(<tbody>)[\s\S]*(</tbody>)"#).unwrap();
    let body = match body_re.find(html) {
        Some(t) => t.as_str(),
        None => panic!("Nothing found"),
    };

    let raw_rows_re = Regex::new(r#"(<tr .*>)([\s\S]*)(</tr>)"#).unwrap();
    let rows = raw_rows_re.captures_iter(body).map(|m| {
        let t = match m.get(1) {
            Some(_m) => _m.as_str(),
            None => "NA",
        };
        t
    }).collect::<Vec<_>>();

    for row in rows {
        println!("rows: {:?}", row);
    }

}

