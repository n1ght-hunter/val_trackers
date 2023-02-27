use std::sync::Arc;

use crate::cookie_store::{CookieStore, CookieStoreRwLock};

pub fn create_client() -> Result<reqwest_impersonate::Client, reqwest_impersonate::Error> {
    let cookie_store = {
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open("cookies.json")
            .map(std::io::BufReader::new)
            .unwrap();
        Arc::new(CookieStoreRwLock::new(
            CookieStore::load_json(file).unwrap(),
        ))
    };
    reqwest_impersonate::Client::builder()
        .danger_accept_invalid_certs(true)
        .chrome_builder(reqwest_impersonate::browser::ChromeVersion::V108)
        .cookie_provider(cookie_store.clone())
        .build()
}
