
use rocket_contrib::json::Json;

use super::models::user::User;
use super::stores::postgres::Tweeter;

#[post("/signup", format="json", data="<user>")]
pub fn signup(conn: Tweeter, user: Json<User>) {
    println!("{:?}", user.0.first_name);
}