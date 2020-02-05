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

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notifications {
    // UserID
    id: String,
    /**
     * Notification holder
     * We use vector, but as we store in serialized
     * the vector item order is persistent.
     * TODO: Check it
     */
    notification: Vec<Notification>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    // Custom notification ID
    id: usize,
    // DateTime created
    date_created: DateTime<Utc>,
    // If it's unread, then it's new
    is_new: bool,
    // Message. Type? Translation?
    subject: String,
    // Location data to create link in GUI
    // e.g.: link to a given issue, or a given product
    // or a given user, or a given order.
    // Type?
    location: Option<Location>,
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
pub struct Location {
    link: Option<String>,
    page: String,
    id: Option<String>,
    section: Option<String>,
}
