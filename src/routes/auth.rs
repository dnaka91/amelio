//! Authentication related routes.

use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use super::NonEmptyString;
use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::hashing;
use crate::services::{self, Credentials, LoginService};
use crate::templates::{self, MessageCode};

/// Login page for any user.
#[get("/login")]
pub fn login(flash: Option<FlashMessage<'_, '_>>) -> templates::Login {
    templates::Login {
        flash: flash.map(|f| {
            (
                f.name().to_owned(),
                f.msg().parse().unwrap_or(MessageCode::Unknown),
            )
        }),
    }
}

/// Credentials of a user to log into the system.
#[derive(FromForm)]
pub struct Login {
    username: NonEmptyString,
    password: NonEmptyString,
}

impl Login {
    fn as_credentials(&self) -> Credentials<'_> {
        Credentials {
            username: &self.username.0,
            password: &self.password.0,
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
    let service = services::login_service(repositories::user_repo(&conn), hashing::new_hasher());

    match service.login(&login.as_credentials()) {
        Ok(id) => {
            cookies.add_private(Cookie::new("session", id.to_string()));
            Ok(Redirect::to(uri!(super::index)))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(login)),
            MessageCode::InvalidCredentials,
        )),
    }
}

/// Logout POST endpoint to handle logout requests.
#[post("/logout")]
pub fn post_logout(mut cookies: Cookies<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("session"));
    Redirect::to(uri!(login))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::uri;

    use crate::tests::check_form;

    #[test]
    fn invalid_post_login() {
        let client = Client::new(crate::rocket().unwrap()).unwrap();
        let uri = uri!(super::post_login).to_string();

        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "username=&password=admin").status()
        );

        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "username=admin&password=").status()
        );
    }
}
