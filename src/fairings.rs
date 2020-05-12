//! Custom fairings (middlewares) that are not specific to any other component.

#![cfg_attr(doc, allow(unused_braces))]

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;
use rocket::{uri, Data, Request, Response};

use crate::routes;

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

use crate::roles::{AdminUser, AuthUser};

const ADMIN_AUTH_PATHS: &[&str] = &["users", "courses"];

/// A fairing that handles authentication and authorization for common routes to save boilerplate
/// code. Without this, several routes must provide an extra route for different authorization
/// levels and unauthenticated users.
///
/// # Warning
///
/// For the fairing to function properly, all routes in [`crate::routes::fairing`] must be mounted
/// at `/` in the Rocket instance.
pub struct Auth;

impl Auth {
    /// Check the request against routes that are only accessible by administrators.
    ///
    /// If the user is logged in but not an admin, he will get a forbidden status response. If the
    /// user is not logged in, he will instead be forwarded to the login page.
    fn check_admin_only_routes(request: &mut Request) {
        let is_auth = request
            .uri()
            .segments()
            .next()
            .map(|seg| ADMIN_AUTH_PATHS.contains(&seg))
            .unwrap_or_default();

        if !is_auth || request.guard::<AdminUser>().is_success() {
            return;
        }

        request.set_method(Method::Get);

        request.set_uri(if request.guard::<&AuthUser>().is_success() {
            uri!(routes::fairing::forbidden)
        } else {
            uri!(routes::fairing::to_login)
        })
    }
}

impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Authentication",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        Self::check_admin_only_routes(request);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use rocket::uri;

    use crate::routes;

    fn prepare_logged_in_client(username: &str, password: &str) -> Client {
        let client = Client::new(crate::rocket().unwrap()).unwrap();

        {
            let res = client
                .post(uri!(routes::auth::login).to_string())
                .body(format!("username={}&password={}", username, password))
                .header(ContentType::Form)
                .dispatch();

            assert_eq!(Status::SeeOther, res.status());
            assert_eq!(Some("/"), res.headers().get_one("Location"));
        }

        client
    }

    #[test]
    fn admin_is_allowed() {
        let client = prepare_logged_in_client("admin", "admin");

        let res = client
            .get(uri!("/users", routes::users::users).to_string())
            .dispatch();

        assert_eq!(Status::Ok, res.status());
    }

    #[test]
    fn student_is_forbidden() {
        let client = prepare_logged_in_client("student1", "student1");

        let res = client
            .get(uri!("/users", routes::users::users).to_string())
            .dispatch();

        assert_eq!(Status::Forbidden, res.status());
    }

    #[test]
    fn anonymous_is_forwarded() {
        let client = Client::new(crate::rocket().unwrap()).unwrap();

        let res = client
            .get(uri!("/users", routes::users::users).to_string())
            .dispatch();

        assert_eq!(Status::SeeOther, res.status());
        assert_eq!(Some("/login"), res.headers().get_one("Location"));
    }
}
