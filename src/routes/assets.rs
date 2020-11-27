//! Routes for loading static assets.

use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;

use rocket::http::hyper::header::{CacheControl, CacheDirective, ETag, EntityTag};
use rocket::http::{ContentType, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::{get, Request, Response};
use rust_embed::RustEmbed;

/// A request guard that extracts the `if-none-match` HTTP header.
///
/// Weak `ETag`s are not handled, as we always return strong ones.
pub struct IfNoneMatch<'a>(&'a str);

impl<'a, 'r> FromRequest<'a, 'r> for IfNoneMatch<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request
            .headers()
            .get_one("if-none-match")
            .map(|t| IfNoneMatch(t.trim_matches('"')))
            .or_forward(())
    }
}

/// Structure that holds all the embedded assets.
#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Access point for all static assets.
///
/// If an `ETag` is found in the request headers and it matches the requested asset, then
/// [`Status::NotModified`] is returned instead of the actual file content.
#[get("/<file..>", rank = 10)]
pub fn get<'r>(file: PathBuf, inm: Option<IfNoneMatch>) -> rocket::response::Result<'r> {
    let file_str = file.to_string_lossy();

    // If the file didn't change, we don't have to send it again.
    if etag_matches(inm, file_str.as_ref()) {
        return Response::build().status(Status::NotModified).ok();
    }

    Assets::get(file_str.as_ref()).map_or_else(
        || Err(Status::NotFound),
        |data| {
            let ext = file.extension().and_then(OsStr::to_str).unwrap_or_default();
            let content_type = ContentType::from_extension(ext).unwrap_or(ContentType::Binary);

            Response::build()
                .header(content_type)
                .header(CacheControl(vec![
                    CacheDirective::Public,
                    CacheDirective::MaxAge(315_360_000),
                ]))
                .header(ETag(EntityTag::new(false, get_etag(file_str))))
                .sized_body(Cursor::new(data))
                .ok()
        },
    )
}

/// Check whether the `ETag` of a `if-none-match` HTTP header matches with the requested asset.
fn etag_matches<F: AsRef<str>>(inm: Option<IfNoneMatch>, file: F) -> bool {
    inm.map(|v| {
        v.0 == *ETAGS
            .get(file.as_ref())
            .unwrap_or(&env!("CARGO_PKG_VERSION"))
    })
    .unwrap_or_default()
}

/// Get the `ETag` for a given asset. If no entry exists, the application version is used instead.
fn get_etag<F: AsRef<str>>(file: F) -> String {
    String::from(
        *ETAGS
            .get(file.as_ref())
            .unwrap_or(&env!("CARGO_PKG_VERSION")),
    )
}
