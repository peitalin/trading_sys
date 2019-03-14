
use std::sync::Arc;


pub fn download_data(coin: &str, start_date: Arc<String>, end_date: Arc<String>) -> Vec<String> {
    let url = format!("https://coinmarketcap.com/currencies/{}/historical-data/?start={}&end={}", coin, start_date, end_date);
    let html = dispatch_request(&url);
    let body = scraper::Html::parse_document(&html);
    let csv_data = extract_data(body);
    csv_data
}

fn dispatch_request(url: &str) -> String {
    let parsed_url = reqwest::Url::parse(url).expect("Bad url format.");
    let mut response = reqwest::get(parsed_url).expect("Failed to get Url");
    println!("Response from url: {}\n\t{}", url, response.status().to_string());
    match response.text() {
        Ok(html) => html,
        Err(e) => panic!("Request Error: {:?}", e),
    }
}

fn extract_data(body: scraper::Html) -> Vec<String> {
    let tr = scraper::Selector::parse("tr").expect("<tr> tags missing!");

    let rows: Vec<String> = body.select(&tr).map(|row| {
        let row_txt = row.text()
            .map(|x| x.trim().replace("*", "").replace(",", ""))
            .filter(|x| x.len() > 0)
            .collect::<Vec<_>>()
            .join(", ");
        row_txt
    }).collect::<Vec<_>>();

    rows
}
