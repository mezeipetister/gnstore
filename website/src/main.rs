// Copyright (C) 2020 Peter Mezei
//
// This file is part of GNStore.
//
// GNStore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// GNStore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNStore.  If not, see <http://www.gnu.org/licenses/>.

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate core_lib;
extern crate crypto;
extern crate ifeq;
extern crate jwt;
extern crate num_format;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_derive;
extern crate storaget;

pub mod controller;
pub mod cors;
pub mod guard;
pub mod login;
pub mod prelude;

use crate::prelude::*;
use core_lib::model::*;
use guard::*;
use rocket::response::NamedFile;
use rocket::Request;
use rocket_cors::AllowedHeaders;
use serde::Serialize;
use std::path::{Path, PathBuf};
use storaget::*;

#[get("/")]
fn index() -> String {
    "Gardenova Welcome".to_owned()
}

#[derive(Debug, Serialize)]
struct ApiWelcomeSchema {
    message: &'static str,
}

#[get("/")]
fn api_welcome(_user: Login) -> StatusOk<ApiWelcomeSchema> {
    StatusOk(ApiWelcomeSchema {
        message: "Welcome to Gardenova API",
    })
}

// #[get("/<file..>")]
// pub fn static_file(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).ok()
// }

#[catch(404)]
fn not_found(_: &Request<'_>) -> ApiError {
    ApiError::NotFound
}

#[catch(401)]
fn unauthorized(_: &Request<'_>) -> ApiError {
    ApiError::Unauthorized
}

#[catch(422)]
fn form_error(_: &Request<'_>) -> ApiError {
    ApiError::InternalError("Minden mező kitöltése kötelező!".to_owned())
}

fn rocket(data: DataLoad) -> rocket::Rocket {
    let mut methods = std::collections::HashSet::new();
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Post));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Get));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Put));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Delete));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Options));

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllOrSome::All,
        allowed_methods: methods,
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .attach(cors)
        // .attach(CORS())
        .manage(data)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                controller::login::post,
                controller::login::reset_password,
                controller::profile::profile_get,
                controller::profile::profile_post,
                controller::profile::password_change,
                controller::notification::notification_get,
                controller::notification::notification_new_get,
                controller::notification::notification_delete,
                controller::notification::notification_seen,
                controller::customer::customer_all_get,
                controller::customer::customer_new_post,
                controller::customer::customer_id_get,
                controller::customer::customer_id_post,
                controller::user::user_all_get,
                controller::user::user_id_get,
                controller::user::user_new_post,
                controller::issue::user_new_put,
                controller::issue::issue_all_get,
                controller::issue::issue_id_get,
                controller::issue::issue_id_follow_post,
                controller::issue::issue_id_unfollow_post,
                controller::issue::issue_id_assign_to_post,
                controller::issue::issue_id_open_post,
                controller::issue::issue_id_close_post,
                controller::issue::issue_id_comment_post,
                controller::issue::issue_id_comment_like_post,
                controller::issue::issue_id_comment_dislike_post,
                controller::issue::issue_id_label_add_post,
                controller::issue::issue_id_label_remove_post,
            ],
        )
        .register(catchers![not_found, unauthorized, form_error])
}

pub struct DataLoad {
    users: Storage<User>,
    notifications: Storage<NotificationContainer>,
    customers: Storage<Customer>,
    issues: Storage<Issue>,
}

fn main() -> StorageResult<()> {
    let data = DataLoad {
        users: Storage::load_or_init::<User>("data/users")?,
        notifications: Storage::load_or_init::<NotificationContainer>("data/notifications")?,
        customers: Storage::load_or_init::<Customer>("data/customers")?,
        issues: Storage::load_or_init::<Issue>("data/issues")?,
    };
    rocket(data).launch();
    Ok(())
}
