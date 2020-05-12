//! User management related routes.

use anyhow::Result;
use log::error;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri, State};

use crate::config::Config;
use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::email;
use crate::hashing;
use crate::models::Role;
use crate::roles::{AdminUser, NoUser};
use crate::services::{self, UserService};
use crate::templates::{self, MessageCode};

/// User management page for administrators.
#[get("/")]
pub fn users(user: AdminUser, conn: DbConn, config: State<Config>) -> Result<templates::Users> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );
    let (active, inactive) = service.list()?;

    Ok(templates::Users {
        role: user.0.role,
        active,
        inactive,
    })
}

/// User creation form for administrators.
#[get("/new")]
pub fn new_user(user: AdminUser, flash: Option<FlashMessage<'_, '_>>) -> templates::NewUser {
    templates::NewUser {
        role: user.0.role,
        flash: flash.map(|f| f.msg().into()),
    }
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
pub fn post_new_user(
    _user: AdminUser,
    data: Form<NewUser>,
    conn: DbConn,
    config: State<Config>,
) -> Result<Redirect, Flash<Redirect>> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );

    match service.create(data.0.username, data.0.name, data.0.role) {
        Ok(()) => Ok(Redirect::to(uri!("/users", users))),
        Err(e) => {
            error!("error during user creation: {:?}", e);
            Err(Flash::error(
                Redirect::to(uri!("/users", new_user)),
                MessageCode::FailedUserCreation,
            ))
        }
    }
}

/// User activation page, only accessible to non-authenticated users.
#[get("/activate/<code>")]
pub fn activate(
    code: String,
    _user: NoUser,
    flash: Option<FlashMessage<'_, '_>>,
) -> templates::ActivateUser {
    templates::ActivateUser {
        flash: flash.map(|f| f.msg().into()),
        code,
    }
}

/// Form data from the user activation form.
#[derive(FromForm)]
pub struct Activate {
    code: String,
    password: String,
}

/// User activation POST endpoint, only accessible to non-authenticated users.
#[post("/activate", data = "<data>")]
pub fn post_activate(
    data: Form<Activate>,
    _user: NoUser,
    conn: DbConn,
    config: State<Config>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );

    match service.activate(&data.code, &data.password) {
        Ok(()) => Ok(Flash::success(
            Redirect::to(uri!(super::auth::login)),
            MessageCode::UserActivated,
        )),
        Err(e) => {
            error!("error during account activation: {:?}", e);
            Err(Flash::error(
                #[allow(non_snake_case)]
                Redirect::to(uri!("/users", activate: data.0.code)),
                MessageCode::InvalidCodeOrError,
            ))
        }
    }
}

/// Enable or disable users as administrator.
#[get("/<id>/enable?<value>")]
pub fn enable_user(
    _user: AdminUser,
    id: i32,
    value: bool,
    conn: DbConn,
    config: State<Config>,
) -> Result<Redirect> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );
    service.enable(id, value)?;

    Ok(Redirect::to(uri!("/users", users)))
}
