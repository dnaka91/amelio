//! User management related routes.

use anyhow::Result;
use rocket::http::Status;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::models::Role;
use crate::roles::{AdminUser, AuthUser};
use crate::services::{self, UserService};
use crate::templates;

/// User management page for administrators.
#[get("/")]
pub fn users_admin(_user: AdminUser, conn: DbConn) -> Result<templates::Users> {
    let user_repo = repositories::user_repo(&conn);
    let service = services::user_service(user_repo);
    let (active, inactive) = service.list()?;

    Ok(templates::Users { active, inactive })
}

/// User management is not allowed for non-admin users.
#[get("/", rank = 2)]
pub const fn users_auth(_user: &AuthUser) -> Status {
    Status::Forbidden
}

/// User management for everyone else, redirecting to the login page.
#[get("/", rank = 3)]
pub fn users() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

/// User creation form for administrators.
#[get("/new")]
pub fn new_user_admin(_user: AdminUser, flash: Option<FlashMessage<'_, '_>>) -> templates::NewUser {
    templates::NewUser {
        flash: flash.map(|f| f.msg().to_owned()),
    }
}

/// User creation is not allowed for non-admin users.
#[get("/new", rank = 2)]
pub const fn new_user_auth(_user: &AuthUser) -> Status {
    Status::Forbidden
}

/// User creation for everyone else, redirecting to the login page.
#[get("/new", rank = 3)]
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
    _user: AdminUser,
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

/// User creation endpoints are not accessible for non-admin users.
#[post("/new", rank = 2)]
pub const fn post_new_user_auth(_user: &AuthUser) -> Status {
    Status::Forbidden
}
/// User creation endpoint for everyone else, redirecting to the login page.
#[post("/new", rank = 3)]
pub fn post_new_user() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}
