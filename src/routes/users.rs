//! User management related routes.

use anyhow::Result;
use log::error;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri, State};

use super::{NonEmptyString, PositiveId, ServerError};
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
pub fn users(
    user: AdminUser,
    conn: DbConn,
    config: State<Config>,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::Users, ServerError> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );
    let (active, inactive) = service.list()?;

    Ok(templates::Users {
        role: user.0.role,
        flash: flash.map(|f| (f.name().to_owned(), f.msg().into())),
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
    username: NonEmptyString,
    name: NonEmptyString,
    role: Role,
}

/// New user POST endpoint to handle user creation, only for administrators.
#[post("/new", data = "<data>")]
pub fn post_new_user(
    _user: AdminUser,
    data: Form<NewUser>,
    conn: DbConn,
    config: State<Config>,
) -> Flash<Redirect> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );

    match service.create(data.0.username.0, data.0.name.0, data.0.role) {
        Ok(()) => Flash::success(
            Redirect::to(uri!("/users", users)),
            MessageCode::UserCreated,
        ),
        Err(e) => {
            error!("error during user creation: {:?}", e);
            Flash::error(
                Redirect::to(uri!("/users", new_user)),
                MessageCode::FailedUserCreation,
            )
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
    code: NonEmptyString,
    password: NonEmptyString,
}

/// User activation POST endpoint, only accessible to non-authenticated users.
#[post("/activate", data = "<data>")]
pub fn post_activate(
    data: Form<Activate>,
    _user: NoUser,
    conn: DbConn,
    config: State<Config>,
) -> Flash<Redirect> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );

    match service.activate(&data.code.0, &data.password.0) {
        Ok(()) => Flash::success(
            Redirect::to(uri!(super::auth::login)),
            MessageCode::UserActivated,
        ),
        Err(e) => {
            error!("error during account activation: {:?}", e);
            Flash::error(
                Redirect::to(uri!("/users", activate: data.0.code.0)),
                MessageCode::InvalidCodeOrError,
            )
        }
    }
}

/// Enable or disable users as administrator.
#[get("/<id>/enable?<value>")]
pub fn enable_user(
    _user: AdminUser,
    id: PositiveId,
    value: bool,
    conn: DbConn,
    config: State<Config>,
) -> Result<Redirect, ServerError> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );
    service.enable(id.0, value)?;

    Ok(Redirect::to(uri!("/users", users)))
}

/// User editing form for administrators.
#[get("/<id>/edit")]
pub fn edit_user(
    user: AdminUser,
    id: PositiveId,
    conn: DbConn,
    config: State<Config>,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::EditUser, ServerError> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );
    let user_data = service.get(id.0)?;

    Ok(templates::EditUser {
        role: user.0.role,
        flash: flash.map(|f| f.msg().into()),
        user: user_data,
    })
}

/// Form data from the user editing form.
#[derive(FromForm)]
pub struct EditUser {
    name: NonEmptyString,
    role: Role,
}

/// Edit user POST endpoint to handle user editing, only for administrators.
#[post("/<id>/edit", data = "<data>")]
pub fn post_edit_user(
    _user: AdminUser,
    id: PositiveId,
    data: Form<EditUser>,
    conn: DbConn,
    config: State<Config>,
) -> Flash<Redirect> {
    let service = services::user_service(
        repositories::user_repo(&conn),
        email::new_smtp_sender(&config.smtp),
        email::new_mail_renderer(&config.host),
        hashing::new_hasher(),
    );

    match service.update(id.0, data.0.name.0, data.0.role) {
        Ok(()) => Flash::success(
            Redirect::to(uri!("/users", users)),
            MessageCode::UserUpdated,
        ),
        Err(e) => {
            error!("error during user update: {:?}", e);
            Flash::error(
                Redirect::to(uri!("/users", edit_user: id)),
                MessageCode::FailedUserUpdate,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::uri;

    use crate::routes::PositiveNum;
    use crate::tests::{check_form, prepare_logged_in_client};

    #[test]
    fn invalid_post_new_user() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = uri!("/users", super::post_new_user).to_string();

        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "username=&name=a&role=student").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "username=a&name=&role=student").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "username=a&name=a&role=").status()
        );
    }

    #[test]
    fn invalid_post_activate() {
        let client = Client::new(crate::rocket().unwrap()).unwrap();
        let uri = uri!(super::post_activate).to_string();

        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=&password=a").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=a&password=").status()
        );
    }

    #[test]
    fn invalid_enable_user_id() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = uri!("/users", super::enable_user: PositiveNum(0), true).to_string();

        assert_eq!(Status::NotFound, client.get(uri).dispatch().status());
    }

    #[test]
    fn invalid_post_edit_user() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = uri!("/users", super::post_edit_user: PositiveNum(1)).to_string();

        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "name=&role=admin").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "name=a&role=test").status()
        );
    }

    #[test]
    fn invalid_edit_user_id() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = uri!("/users", super::edit_user: PositiveNum(0)).to_string();

        assert_eq!(Status::NotFound, client.get(uri).dispatch().status());
    }
}
