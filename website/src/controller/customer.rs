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
use core_lib::customer::*;
use core_lib::model::customer::*;
use core_lib::prelude::AppResult;
use core_lib::user;
use core_lib::user::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerResponse {
    id: String,
    date_created: DateTime<Utc>,
    created_by: String,
    name: String,
    address: CustomerAddress,
    email: String,
    phone: String,
    tax_number: String,
    has_user: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerAddress {
    zip: String,
    location: String,
    address: String,
}

impl<T> From<&T> for CustomerResponse
where
    T: Customer,
{
    fn from(c: &T) -> Self {
        CustomerResponse {
            id: c.get_id(),
            date_created: c.get_date_created(),
            created_by: c.get_created_by(),
            name: c.get_name(),
            address: CustomerAddress {
                zip: c.get_address().0,
                location: c.get_address().1,
                address: c.get_address().2,
            },
            email: c.get_email(),
            phone: c.get_phone(),
            tax_number: c.get_tax_number(),
            has_user: c.has_user(),
        }
    }
}

#[get("/customer/all")]
pub fn customer_all_get(
    _: Login,
    data: State<DataLoad>,
) -> Result<StatusOk<Vec<CustomerResponse>>, ApiError> {
    let res = data
        .inner()
        .customers
        .into_iter()
        .map(|d| d.get(|c| c.into()))
        .collect::<Vec<CustomerResponse>>();
    Ok(StatusOk(res))
}

// #[get("/notification/new")]
// pub fn notification_new_get(user: Login, data: State<DataLoad>) -> Result<StatusOk<()>, ApiError> {
//     let mut notification = NotificationV1::new("Hello bello".to_owned());
//     notification.set_location(LocationV1::Raw("Demo location".to_owned()));
//     match data.inner().notifications.get_by_id(user.userid()) {
//         Ok(container) => {
//             container.update(|c| c.add(notification.clone()));
//             Ok(StatusOk(()))
//         }
//         Err(_) => {
//             data.inner()
//                 .notifications
//                 .add_to_storage(NotificationContainerV1::new(user.userid().to_string()))
//                 .unwrap();
//             Ok(StatusOk(()))
//         }
//     }
// }

// #[delete("/notification/<id>")]
// pub fn notification_delete(
//     user: Login,
//     data: State<DataLoad>,
//     // Notification ID
//     id: usize,
// ) -> Result<StatusOk<()>, ApiError> {
//     match data.inner().notifications.get_by_id(user.userid()) {
//         Ok(container) => {
//             container.update(|c| c.remove_by_id(id))?;
//             Ok(StatusOk(()))
//         }
//         Err(_) => Err(ApiError::BadRequest(
//             "Értesítés azonosító nem található".to_owned(),
//         )),
//     }
// }

// #[put("/notification/<id>/seen")]
// pub fn notification_seen(
//     user: Login,
//     data: State<DataLoad>,
//     // Notification ID
//     id: usize,
// ) -> Result<StatusOk<()>, ApiError> {
//     match data.inner().notifications.get_by_id(user.userid()) {
//         Ok(container) => {
//             container.update(|c| {
//                 if let Some(notification) = c.get_by_id(id) {
//                     notification.set_seen();
//                 }
//             });
//             Ok(StatusOk(()))
//         }
//         Err(_) => Err(ApiError::BadRequest(
//             "Értesítés azonosító nem található".to_owned(),
//         )),
//     }
// }