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

use crate::model::version::issue::event::v1::Event;
use crate::model::version::issue::label::v1::Label;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    /**
     * ID
     */
    pub id: String,
    /**
     * Issue title
     */
    pub title: String,
    /**
     * Issue description
     * TODO: should be markdown capable
     */
    pub description: String,
    /**
     * Date created, Chrono DateTime<Utc>
     */
    pub date_created: DateTime<Utc>,
    /**
     * Created by @userid
     */
    pub created_by: String,
    /**
     * Assigned label list
     */
    pub labels: Vec<Label>,
    /**
     * Assigned to @userid
     */
    pub assigned_to: String,
    /**
     * Event list
     */
    pub events: Vec<Event>,
    /**
     * Number of comments added
     */
    pub comment_count: usize,
    /**
     * Followed by Vec<@userid: String>
     */
    pub followed_by: Vec<String>,
    /**
     * Status field
     * true if open, false if closed issue
     */
    pub is_open: bool,
}

// Implement StorageObject for Issue
impl VecPackMember for Issue {
    fn get_id(&self) -> &str {
        &self.id
    }
}
