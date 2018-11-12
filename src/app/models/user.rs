/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */
extern crate rand;
extern crate argon2rs;

use rand::prng::isaac64::Isaac64Rng;
use rand::RngCore;
use std::str;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub salt: Option<String>,
}

impl User {
    pub fn new(first_name: String, last_name: String, password: String) -> User {
        return User {
            first_name,
            last_name,
            password,
            salt: None
        };
    }

    pub fn encrypt_password(&mut self) {
        let salt = Isaac64Rng::new_from_u64(rand::random()).next_u64().to_string();
        let password: String = argon2rs::argon2i_simple(&self.password, &salt).iter().map(|b|
            format!("{:02x}", b)
        ).collect();
        self.salt = Some(salt);
        self.password = password;
    }

    pub fn decrypt_password(&self) {

    }
}