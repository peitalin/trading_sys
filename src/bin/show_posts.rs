extern crate diesel;
extern crate trading_sys;

use diesel::prelude::*;

use trading_sys::models::{NewPost, Post};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use trading_sys::schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    use trading_sys::schema::posts::dsl::*;

    let connection = trading_sys::establish_connection_pg();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("\n{}", post.title);
        println!("-------\n");
        println!("{}\n", post.body);
    }
}
