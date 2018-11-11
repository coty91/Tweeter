/**
 * Author: Coty A. Rothery
 * Date: 11/11/2018
 */

#[database("tweeter")]
pub struct Tweeter(postgres::Connection);
