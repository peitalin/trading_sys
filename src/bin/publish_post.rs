extern crate diesel;
extern crate trading_sys;

use self::diesel::prelude::*;
use self::models::Post;
use self::trading_sys::*;
use std::env::args;

fn main() {
    use trading_sys::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = trading_sys::establish_connection_pg();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
