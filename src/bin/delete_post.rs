extern crate diesel;
extern crate trading_sys;
use self::trading_sys::establish_connection_pg;

use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use trading_sys::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection_pg();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");
    println!("Deleted {} posts", num_deleted);
}
