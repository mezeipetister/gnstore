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
use core_lib::model::*;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationResponse {
    id: usize,
    date_created: DateTime<Utc>,
    is_new: bool,
    subject: String,
    location: Option<String>,
}

impl From<&Notification> for NotificationResponse {
    fn from(from: &Notification) -> Self {
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
pub fn notification_get(
    user: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<NotificationResponse>>, ApiError> {
    match data.inner().notifications.get_by_id(user.userid()) {
        Ok(object) => {
            let mut response = object
                .get(|n| n.get_notifications().to_vec())
                .iter()
                .map(|v| v.into())
                .collect::<Vec<NotificationResponse>>();
            /*
             * Order result by date
             */
            response.sort_by(|a, b| b.date_created.cmp(&a.date_created));
            Ok(StatusOk(response))
        }
        Err(_) => Ok(StatusOk(Vec::new())),
    }
}

#[get("/notification/new")]
pub fn notification_new_get(user: Login, data: State<DataLoad>) -> Result<StatusOk<()>, ApiError> {
    let mut notification = Notification::new("Hello bello".to_owned());
    notification.set_location(Location::Raw("Demo location".to_owned()));
    match data.inner().notifications.get_by_id(user.userid()) {
        Ok(container) => {
            container.update(|c| c.add(notification.clone()));
            Ok(StatusOk(()))
        }
        Err(_) => {
            data.inner()
                .notifications
                .add_to_storage(NotificationContainer::new(user.userid().to_string()))
                .unwrap();
            Ok(StatusOk(()))
        }
    }
}

#[delete("/notification/<id>")]
pub fn notification_delete(
    user: Login,
    data: State<DataLoad>,
    // Notification ID
    id: usize,
) -> Result<StatusOk<()>, ApiError> {
    match data.inner().notifications.get_by_id(user.userid()) {
        Ok(container) => {
            container.update(|c| c.remove_by_id(id))?;
            Ok(StatusOk(()))
        }
        Err(_) => Err(ApiError::BadRequest(
            "Értesítés azonosító nem található".to_owned(),
        )),
    }
}

#[put("/notification/<id>/seen")]
pub fn notification_seen(
    user: Login,
    data: State<DataLoad>,
    // Notification ID
    id: usize,
) -> Result<StatusOk<()>, ApiError> {
    match data.inner().notifications.get_by_id(user.userid()) {
        Ok(container) => {
            container.update(|c| {
                if let Some(notification) = c.get_by_id(id) {
                    notification.set_seen();
                }
            });
            Ok(StatusOk(()))
        }
        Err(_) => Err(ApiError::BadRequest(
            "Értesítés azonosító nem található".to_owned(),
        )),
    }
}
