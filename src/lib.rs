/* --- MODULES --- */

pub mod client;
pub mod model;

/* --- ------- --- */



/* --- ENUMS --- */

#[derive(Debug)]
pub enum Error {
    HTTPError(reqwest::Error),
    ParseError(serde_json::Error),
    ClientError(crate::model::ClientError),
}

/* --- ----- --- */
