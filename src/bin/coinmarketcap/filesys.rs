
pub fn create_data_directory() {
    let date = chrono::Utc::now().format("%Y-%m-%d");
    println!("Creating directory: {}", &date);
    let _ = std::fs::create_dir(std::path::Path::new(&format!("./data/coinmarketcap/{}", &date)));
}


pub fn check_if_filepath_exists(filename: &str) -> bool {
    match std::fs::File::open(&filename) {
        Ok(_) => {
            println!("File exists, skipping.");
            true
        },
        Err(_) => false,
    }
}

pub fn create_filepath(rank: &i32, id: &str) -> String {
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let fp = match rank {
        n if *n < 10 => format!("./data/{}/00{}_{}.csv", &date, rank, id),
        n if (10 <= *n) && (*n < 100) => format!("./data/{}/0{}_{}.csv", &date, rank, id),
        _ => format!("./data/{}/{}_{}.csv", &date, rank, id),
    };
    fp
}
