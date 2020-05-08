//! User management related routes.

use anyhow::Result;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::models::Role;
use crate::roles::AuthUser;
use crate::services::{self, UserService};
use crate::templates;

/// User management page for administrators.
#[get("/")]
pub fn users_admin(_user: &AuthUser, conn: DbConn) -> Result<templates::Users> {
    let user_repo = repositories::user_repo(&conn);
    let service = services::user_service(user_repo);
    let (active, inactive) = service.list()?;

    Ok(templates::Users { active, inactive })
}

/// User management for everyone else, redirecting to the login page.
#[get("/", rank = 2)]
pub fn users() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

/// User creation form for administrators.
#[get("/new")]
pub fn new_user_admin(_user: &AuthUser, flash: Option<FlashMessage<'_, '_>>) -> templates::NewUser {
    templates::NewUser {
        flash: flash.map(|f| f.msg().to_owned()),
    }
}

/// User creation for everyone else, redirecting to the login page.
#[get("/new", rank = 2)]
pub fn new_user() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

/// Form data from the user creation form.
#[derive(FromForm)]
pub struct NewUser {
    username: String,
    name: String,
    role: Role,
}

/// New user POST endpoint to handle user creation, only for administrators.
#[post("/new", data = "<data>")]
pub fn post_new_user_admin(
    _user: &AuthUser,
    data: Form<NewUser>,
    conn: DbConn,
) -> Result<Redirect, Flash<Redirect>> {
    let user_repo = repositories::user_repo(&conn);
    let service = services::user_service(user_repo);

    match service.create(data.0.username, data.0.name, data.0.role) {
        Ok(()) => Ok(Redirect::to(uri!(users))),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(new_user)),
            "Failed creating user.",
        )),
    }
}

/// User creation endpoint for everyone else, redirecting to the login page.
#[post("/new", rank = 2)]
pub fn post_new_user() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}
