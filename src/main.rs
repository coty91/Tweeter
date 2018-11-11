#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate uuid;

pub mod app;

use self::app::routes;
use self::app::stores;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(stores::postgres::Tweeter::fairing())
        .mount("/", routes![
            routes::home::index, 
            routes::user::signup
        ])
}

fn main() {
    rocket().launch();
}
