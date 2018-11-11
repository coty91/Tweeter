/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */

use rocket_contrib::json::Json;

use super::models::tweet::Tweets;
use super::stores::postgres::Tweeter;

#[get("/", format = "json")]
pub fn index(conn: Tweeter) -> Json<Tweets> {
    let mut tweets: Tweets = Tweets {
        data: Vec::new()
    };
    for row in &conn.query("select tweet from tweets", &[]).unwrap() {
        tweets.data.push(row.get("tweet"));
    }
    return Json(tweets);
}