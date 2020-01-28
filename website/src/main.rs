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

pub mod cors;
pub mod guard;
pub mod login;
pub mod prelude;

use crate::prelude::CheckError;
use crate::prelude::FlashOk;
use chrono::prelude::*;
use core_lib::prelude::AppResult;
use core_lib::user;
use core_lib::user::User;
use core_lib::user::UserV1;
use core_lib::user::*;
use cors::CORS;
use guard::*;
use login::*;
use prelude::Check;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::Request;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use storaget::*;

#[derive(Serialize)]
struct Response<'a, T: 'a> {
    api_version: &'a str,
    response: T,
}

#[get("/")]
fn index(_user: Login) -> JsonValue {
    json!({"status": "GNStore API"})
}

#[derive(Serialize, Deserialize)]
struct FormLogin {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
fn login(login: Json<FormLogin>, data: State<DataLoad>) -> Result<JsonValue, Status> {
    // Temp login to admin
    // TODO: Remove this part, vulnerable code
    if login.username == "admin".to_owned() && login.password == "admin".to_owned() {
        return Ok(json!({"token": create_token(&login.username)}));
    } else {
        return Err(Status::Unauthorized);
    }
}

#[get("/long")]
fn get_long(_user: Login) -> JsonValue {
    std::thread::sleep(std::time::Duration::from_secs(3));
    json!({"msg": "It was long!"})
}

#[get("/quick")]
fn get_quick(_user: Login) -> JsonValue {
    json!({"msg": "It was quick!"})
}

#[get("/private")]
fn private(user: Login) -> JsonValue {
    json!({ "msg": format!("Ok, {}", user.userid()) })
}

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(_: &Request<'_>) -> JsonValue {
    json!({"status":"Request not found"})
}

#[catch(401)]
fn unauthorized(_: &Request<'_>) -> JsonValue {
    json!({"status":"UnAuthorized"})
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
        .mount("/", routes![index, get_long, get_quick, login, private])
        .register(catchers![not_found, unauthorized])
}

struct DataLoad {
    users: Storage<UserV1>,
}

fn main() -> StorageResult<()> {
    let data = DataLoad {
        users: Storage::load_or_init::<UserV1>("data/users")?,
    };
    rocket(data).launch();
    Ok(())
}
