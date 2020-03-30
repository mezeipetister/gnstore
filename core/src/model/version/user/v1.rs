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

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password_hash: String,
    pub date_created: DateTime<Utc>,
    pub created_by: String,
    pub customers: Vec<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: String::default(),
            name: String::default(),
            email: String::default(),
            phone: String::default(),
            password_hash: String::default(),
            date_created: Utc::now(),
            created_by: String::default(),
            customers: Vec::new(),
        }
    }
}

impl VecPackMember for User {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl TryFrom for User {
    type TryFrom = User;
}
