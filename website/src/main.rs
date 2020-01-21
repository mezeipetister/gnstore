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
extern crate serde_derive;
extern crate storaget;

pub mod cors;
pub mod guard;
pub mod login;
pub mod prelude;

use crate::core_lib::Account;
use crate::core_lib::Transaction;
use crate::prelude::CheckError;
use crate::prelude::FlashOk;
use chrono::prelude::*;
use core_lib::prelude::AppResult;
use core_lib::user;
use core_lib::user::User;
use core_lib::user::UserV1;
use core_lib::user::*;
use core_lib::Account1;
use core_lib::Transaction1;
use cors::CORS;
use guard::*;
use login::*;
use prelude::{Check, FlashRedirect};
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket::Request;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use storaget::*;

#[get("/")]
fn index() -> JsonValue {
    json!({"status": "HelloWorld"})
}

#[get("/long")]
fn get_long() -> JsonValue {
    std::thread::sleep(std::time::Duration::from_secs(3));
    json!({"msg": "It was long!"})
}

#[get("/quick")]
fn get_quick() -> JsonValue {
    json!({"msg": "It was quick!"})
}

// #[get("/")]
// fn index(user: Login, flash: Option<FlashMessage>) -> JsonValue {
//     json!({"status": "HelloWorld"})
// }

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> JsonValue {
    json!({"status":"Not found"})
}

#[catch(401)]
fn unauthorized(req: &Request<'_>) -> Flash<Redirect> {
    Flash::new(
        Redirect::to("/login"),
        "LOGIN_REDIRECT_TO",
        req.route().unwrap().uri.path(),
    )
}

fn rocket(data: DataLoad) -> rocket::Rocket {
    rocket::ignite()
        .attach(CORS())
        .manage(data)
        .mount("/", routes![index, get_long, get_quick])
        .register(catchers![not_found, unauthorized])
}

struct DataLoad {
    users: Storage<UserV1>,
    accounts: Storage<Account1>,
    transactions: Storage<Transaction1>,
}

fn main() -> StorageResult<()> {
    let data = DataLoad {
        users: Storage::load_or_init::<UserV1>("data/users")?,
        accounts: Storage::load_or_init::<Account1>("data/accounts")?,
        transactions: Storage::load_or_init::<Transaction1>("data/transactions")?,
    };
    rocket(data).launch();
    Ok(())
}
