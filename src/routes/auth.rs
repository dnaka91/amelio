//! Authentication related routes.

use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::services::{self, Credentials, LoginService};
use crate::templates;

/// Login page for any user.
#[get("/login")]
pub fn login(flash: Option<FlashMessage<'_, '_>>) -> templates::Login {
    templates::Login {
        flash: flash.map(|f| (f.name().to_owned(), f.msg().to_owned())),
    }
}

/// Credentials of a user to log into the system.
#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

impl Login {
    fn as_credentials(&self) -> Credentials {
        Credentials {
            username: &self.username,
            password: &self.password,
        }
    }
}

/// Login POST endpoint to handle login requests.
#[post("/login", data = "<login>")]
pub fn post_login(
    mut cookies: Cookies<'_>,
    login: Form<Login>,
    conn: DbConn,
) -> Result<Redirect, Flash<Redirect>> {
    let user_repo = repositories::user_repo(&conn);
    let service = services::login_service(user_repo);

    match service.login(&login.as_credentials()) {
        Ok(id) => {
            cookies.add_private(Cookie::new("session", id.to_string()));
            Ok(Redirect::to(uri!(super::index)))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(login)),
            "Invalid username or password.",
        )),
    }
}

/// Logout POST endpoint to handle logout requests.
#[post("/logout")]
pub fn post_logout(mut cookies: Cookies<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("session"));
    Redirect::to(uri!(login))
}
