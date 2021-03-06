#![feature(proc_macro_hygiene, decl_macro)]
/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod app;

use self::app::routes;
use self::app::stores;
use self::app::catchers;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(stores::postgres::Tweeter::fairing())
        .register(catchers![
            catchers::not_found,
            catchers::access_forbidden,
            catchers::unprocessable_entity
        ])
        .mount("/", routes![
            routes::home::index, 
            routes::user::signup
        ])
}

fn main() {
    rocket().launch();
}
