//! Custom fairings (middlewares) that are not specific to any other component.

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

const CSP_HEADER_NAME: &str = "Content-Security-Policy";

/// A fairing that injects the `Content-Security-Policy` header if it's not already present in the
/// HTTP response.
pub struct Csp;

impl Fairing for Csp {
    fn info(&self) -> Info {
        Info {
            name: "Content Security Policy",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        if response.headers().contains(CSP_HEADER_NAME) {
            return;
        }

        response.set_raw_header(
            CSP_HEADER_NAME,
            "\
            default-src 'none'; \
            img-src 'self'; \
            script-src 'self' https://cdn.jsdelivr.net; \
            style-src https://cdn.jsdelivr.net; \
            font-src https://cdn.jsdelivr.net; \
            base-uri 'none'; \
            form-action 'self'; \
            frame-ancestors 'none'\
        ",
        );
    }
}
