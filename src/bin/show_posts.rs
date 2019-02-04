

extern crate diesel;
extern crate trading_sys;

use diesel::prelude::*;



use self::models::{ Post, NewPost, TradeData };
pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
fn main() {
    use schema::posts::dsl::*;


    let connection = establish_connection_pg();

    let results = posts.filter(published.eq(true))
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
