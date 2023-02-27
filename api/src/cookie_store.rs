use std::sync::{LockResult, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use bytes::Bytes;
use cookie::{Cookie as RawCookie, ParseError as RawCookieParseError};
pub use cookie_store::CookieStore;
use reqwest_impersonate::header::HeaderValue;
use url;

fn set_cookies(
    cookie_store: &mut CookieStore,
    cookie_headers: &mut dyn Iterator<Item = &HeaderValue>,
    url: &url::Url,
) {
    let cookies = cookie_headers.filter_map(|val| {
        std::str::from_utf8(val.as_bytes())
            .map_err(RawCookieParseError::from)
            .and_then(RawCookie::parse)
            .map(|c| c.into_owned())
            .ok()
    });
    cookie_store.store_response_cookies(cookies, url);

    let mut writer = std::fs::File::create("cookies.json")
        .map(std::io::BufWriter::new)
        .unwrap();
    cookie_store.save_json(&mut writer).unwrap();
}

fn cookies(cookie_store: &CookieStore, url: &url::Url) -> Option<HeaderValue> {
    let s = cookie_store
        .get_request_values(url)
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("; ");

    if s.is_empty() {
        return None;
    }

    HeaderValue::from_maybe_shared(Bytes::from(s)).ok()
}

/// A [`cookie_store::CookieStore`] wrapped internally by a [`std::sync::RwLock`], suitable for use in
/// async/concurrent contexts.
#[derive(Debug)]
pub struct CookieStoreRwLock(RwLock<CookieStore>);

impl Default for CookieStoreRwLock {
    /// Create a new, empty [`CookieStoreRwLock`].
    fn default() -> Self {
        CookieStoreRwLock::new(CookieStore::default())
    }
}

impl CookieStoreRwLock {
    /// Create a new [`CookieStoreRwLock`] from an existing [`cookie_store::CookieStore`].
    pub fn new(cookie_store: CookieStore) -> CookieStoreRwLock {
        CookieStoreRwLock(RwLock::new(CookieStore::default()))
    }

    /// Lock and get a read (non-exclusive) handle to the contained [`cookie_store::CookieStore`].
    pub fn read(
        &self,
    ) -> Result<RwLockReadGuard<'_, CookieStore>, PoisonError<RwLockReadGuard<'_, CookieStore>>>
    {
        self.0.read()
    }

    /// Lock and get a write (exclusive) handle to the contained [`cookie_store::CookieStore`].
    pub fn write(
        &self,
    ) -> Result<RwLockWriteGuard<'_, CookieStore>, PoisonError<RwLockWriteGuard<'_, CookieStore>>>
    {
        self.0.write()
    }

    /// Consume this [`CookieStoreRwLock`], returning the underlying [`cookie_store::CookieStore`]
    pub fn into_inner(self) -> LockResult<CookieStore> {
        self.0.into_inner()
    }
}

impl reqwest_impersonate::cookie::CookieStore for CookieStoreRwLock {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        let mut write = self.0.write().unwrap();
        set_cookies(&mut write, cookie_headers, url);
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        let read = self.0.read().unwrap();
        cookies(&read, url)
    }
}
