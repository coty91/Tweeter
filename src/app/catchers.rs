/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */

use rocket::response::content;

#[catch(422)]
pub fn unprocessable_entity(req: &rocket::Request) -> content::Json<String> {
    content::Json("{
        \"code\": 422,
        \"message\": \"Unprocessable Entity\"
    }".to_string())
}

#[catch(404)]
pub fn not_found(req: &rocket::Request) -> content::Json<String> {
    content::Json("{
        \"code\": 404,
        \"message\": \"Not Found\"
    }".to_string())
}

#[catch(403)]
pub fn access_forbidden(req: &rocket::Request) -> content::Json<String> {
    content::Json("{
        \"code\": 403,
        \"message\": \"Access Forbidden\"
    }".to_string())
}