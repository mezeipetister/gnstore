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

use crate::prelude::AppResult;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Notifications<T>
where
    T: Notification + Serialize + Debug,
{
    /// Remove notification by id
    fn remove_by_id(&mut self, id: usize) -> AppResult<()>;
    /// Add new notification
    fn add(&mut self, notification: T) -> AppResult<()>;
    /// Get notification vector
    fn get_notifications(&self) -> Vec<T>;
}

pub trait Notification {
    /// Set notification to be seen
    /// Status change
    fn set_seen(&mut self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notifications1 {
    /**
     * UserID => NotificationID
     * We use the same userID here,
     * as each user has just maximum
     * one Notification holder
     *
     * If a user has a notification holder,
     * that could be just one here
     *
     */
    id: String,
    /**
     * Notification holder
     * We use vector, but as we store in serialized
     * the vector item order is persistent.
     * TODO: Check it
     */
    notification: Vec<Notification1>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification1 {
    /**
     * Custom notification ID
     */
    id: usize,
    /**
     * DateTime created
     */
    date_created: DateTime<Utc>,
    /**
     * If it's unread, then it's new
     * it's false after seen
     */
    is_new: bool,
    /**
     * Message. Type? Translation?
     */
    subject: String,
    /**
     * Location data to create link in GUI
     * e.g.: link to a given issue, or a given product
     * or a given user, or a given order.
     * Type?
     */
    location: Option<Location1>,
}

/// E.g.:
/// Location {
///     page: "issue",
///     id: "14",
///     section: "19"
/// }
///
/// => /a/issue/14#19
#[derive(Serialize, Deserialize, Debug)]
pub struct Location1 {
    page: String,
    id: Option<String>,
    section: Option<String>,
}
