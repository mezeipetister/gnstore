// Copyright (C) 2019 Peter Mezei
//
// This file is part of Project A.
//
// Project A is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2; of the License, or
// (at your option) any later version.
//
// Project A is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Project A.  If not, see <http://www.gnu.org/licenses/>.

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate core_lib;
extern crate ifeq;
extern crate num_format;
#[macro_use]
extern crate rocket_contrib;
extern crate crypto;
extern crate jwt;
extern crate rocket_cors;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_derive;
extern crate storaget;

pub mod controller;
pub mod cors;
pub mod guard;
pub mod login;
pub mod model;
pub mod prelude;

// use core_lib::prelude::AppResult;
// use core_lib::user;
// use core_lib::user::User;
use core_lib::user::UserV1;
// use core_lib::user::*;
use guard::*;
// use login::*;
use crate::prelude::*;
use core_lib::model::notification::notification_v1::*;
use core_lib::notification::*;
use rocket::http::Method::*;
use rocket::Request;
use rocket::Route;
use rocket_cors::AllowedHeaders;
use serde::Serialize;
use storaget::*;

#[derive(Debug, Serialize)]
struct User {
    name: String,
}

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

// #[get("/long")]
// fn get_long(_user: Login) -> JsonValue {
//     std::thread::sleep(std::time::Duration::from_secs(3));
//     json!({"msg": "It was long!"})
// }

#[get("/quick")]
fn get_quick() -> Result<StatusCreated<User>, ApiError> {
    Ok(StatusCreated(User {
        name: "Peti".to_owned(),
    }))
    // Err(ApiError::BadRequest("Oooo"))
}

// #[get("/private")]
// fn private(user: Login) -> JsonValue {
//     json!({ "msg": format!("Ok, {}", user.userid()) })
// }

// #[get("/static/<file..>")]
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
fn formError(_: &Request<'_>) -> ApiError {
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
                controller::notification::profile_get,
                controller::notification::profile_new_get
            ],
        )
        .register(catchers![not_found, unauthorized, formError])
}

pub struct DataLoad {
    users: Storage<UserV1>,
    notifications: Storage<NotificationContainerV1>,
}

fn main() -> StorageResult<()> {
    let data = DataLoad {
        users: Storage::load_or_init::<UserV1>("data/users")?,
        notifications: Storage::load_or_init::<NotificationContainerV1>("data/notifications")?,
    };
    rocket(data).launch();
    Ok(())
}
