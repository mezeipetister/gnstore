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
use chrono::prelude::*;
use core_lib::model::notification::*;
use core_lib::notification::*;
use core_lib::prelude::AppResult;
use core_lib::user;
use core_lib::user::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationResponse {
    id: usize,
    date_created: DateTime<Utc>,
    is_new: bool,
    subject: String,
    location: Option<String>,
}

impl<T> From<&T> for NotificationResponse
where
    T: Notification,
{
    fn from(from: &T) -> Self {
        NotificationResponse {
            id: from.get_id(),
            date_created: from.get_date_created(),
            is_new: from.get_is_new(),
            subject: from.get_subject().to_string(),
            location: from.get_location(),
        }
    }
}

#[get("/notification")]
pub fn profile_get(
    user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<NotificationResponse>>, ApiError> {
    match data.inner().notifications.get_by_id(user.userid()) {
        Ok(object) => {
            let response = object
                .get(|n| n.get_notifications().to_vec())
                .iter()
                .map(|v| v.into())
                .collect::<Vec<NotificationResponse>>();
            Ok(StatusOk(response))
        }
        Err(_) => Ok(StatusOk(Vec::new())),
    }
}

#[get("/notification/new")]
pub fn profile_new_get(user: Login, data: State<DataLoad>) -> Result<StatusOk<()>, ApiError> {
    let mut notification = NotificationV1::new("Hello bello".to_owned());
    notification.set_location(LocationV1::Raw("Demo location".to_owned()));
    match data.inner().notifications.get_by_id(user.userid()) {
        Ok(container) => {
            container.update(|c| c.add(notification.clone()));
            Ok(StatusOk(()))
        }
        Err(_) => {
            data.inner()
                .notifications
                .add_to_storage(NotificationContainerV1::new(user.userid().to_string()))
                .unwrap();
            Ok(StatusOk(()))
        }
    }
}
