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

use crate::guard::Login;
use crate::prelude::*;
use crate::DataLoad;
use core_lib::prelude::AppResult;
use core_lib::user;
use core_lib::user::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    username: Option<String>,
    email: Option<String>,
    name: Option<String>,
}

#[get("/profile")]
pub fn profile_get(user: Login, data: State<DataLoad>) -> Result<StatusOk<Profile>, ApiError> {
    match user::get_user_by_id(&data.inner().users, &user.userid()) {
        Ok(usr) => {
            let profile = usr.update(|user| -> Profile {
                Profile {
                    username: Some(user.get_user_id().to_owned()),
                    email: Some(user.get_user_email().to_owned()),
                    name: Some(user.get_user_name().to_owned()),
                }
            });
            return Ok(StatusOk(profile));
        }
        Err(_) => {
            return Err(ApiError::InternalError(
                "A felhasználó nem található.".to_owned(),
            ))
        }
    };
}

#[post("/profile", data = "<form>")]
pub fn profile_post(
    user: Login,
    data: State<DataLoad>,
    form: Json<Profile>,
) -> Result<StatusOk<Profile>, ApiError> {
    let _ = match &form.username {
        Some(u) => u,
        None => return Err(ApiError::BadRequest("Hiányzó usernév mező".to_owned())),
    };
    let email = match &form.email {
        Some(e) => e,
        None => return Err(ApiError::BadRequest("Hiányzó email mező".to_owned())),
    };
    let name = match &form.name {
        Some(n) => n,
        None => return Err(ApiError::BadRequest("Hiányzó név mező".to_owned())),
    };
    match user::get_user_by_id(&data.inner().users, &user.userid()) {
        Ok(usr) => {
            let res = usr.update(|user| -> AppResult<()> {
                user.set_user_name(name.clone())?;
                user.set_user_email(email.clone())?;
                Ok(())
            });
            match res {
                Ok(_) => {
                    let p = Profile {
                        username: Some(user.userid().to_owned()),
                        email: Some(email.clone()),
                        name: Some(name.clone()),
                    };
                    return Ok(StatusOk(p));
                }
                Err(_) => return Err(ApiError::BadRequest("Hibás adatok".to_owned())),
            }
        }
        Err(_) => {
            return Err(ApiError::InternalError(
                "A felhasználó nem található.".to_owned(),
            ))
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPassword {
    password1: Option<String>,
    password2: Option<String>,
}

#[post("/profile/new_password", data = "<form>")]
pub fn password_change(
    user: Login,
    data: State<DataLoad>,
    form: Json<NewPassword>,
) -> Result<StatusOk<()>, ApiError> {
    let password1 = match &form.password1 {
        Some(p) => p,
        None => return Err(ApiError::BadRequest("Hiányzó adatmező!".to_owned())),
    };
    let password2 = match &form.password2 {
        Some(p) => p,
        None => return Err(ApiError::BadRequest("Hiányzó adatmező!".to_owned())),
    };
    if password1 != password2 {
        return Err(ApiError::BadRequest(
            "A két jelszó nem egyezik meg egymással".to_owned(),
        ));
    }
    match user::get_user_by_id(&data.inner().users, &user.userid()) {
        Ok(usr) => usr.update(|u| u.set_password(password1.clone()))?,
        Err(_) => return Err(ApiError::InternalError("Azonosítási hiba".to_owned())),
    }
    Ok(StatusOk(()))
}
