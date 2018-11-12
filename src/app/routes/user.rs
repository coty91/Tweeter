/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */

use rocket_contrib::json::Json;

use super::models::user::User;
use super::stores::postgres::Tweeter;

#[post("/signup", format="json", data="<user_data>")]
pub fn signup(conn: Tweeter, user_data: Json<User>) {
    let mut user: User = User::new(
        user_data.0.first_name, 
        user_data.0.last_name, 
        user_data.0.password
    );

    user.encrypt_password();

    conn.execute("INSERT INTO users (first_name, last_name, password, salt) VALUES ($1, $2, $3, $4)", &[
        &user.first_name, &user.last_name, &user.password, &user.salt
    ]).unwrap();
}