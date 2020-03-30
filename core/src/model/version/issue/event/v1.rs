// Copyright (C) 2020 peter
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

pub use crate::model::version::issue::event::eventkind::v1::EventKind;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    /**
     * Event created at DateTime<Utc>
     */
    pub date_created: DateTime<Utc>,
    /**
     * Event created by
     */
    pub created_by: String,
    /**
     * EventKind stored here
     * This contains all the details
     */
    pub kind: EventKind,
}

impl Event {
    pub fn new(created_by: String, kind: EventKind) -> Self {
        Event {
            date_created: Utc::now(),
            created_by,
            kind,
        }
    }
}
