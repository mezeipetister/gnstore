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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    /// ID for customer
    pub id: String,
    /// Vector of usernames
    pub related_users: Vec<String>,
    pub name: String,
    pub tax_number: String,
    pub address: InvoiceAddress,
    pub phone: String,
    pub email: String,
    pub date_created: DateTime<Utc>,
    /// Username who created
    pub created_by: String,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            id: String::new(),
            related_users: Vec::new(),
            name: String::new(),
            tax_number: String::new(),
            address: InvoiceAddress::default(),
            phone: String::new(),
            email: String::new(),
            date_created: Utc::now(),
            created_by: String::new(),
        }
    }
}

// Implement StorageObject for NotificationContainer
impl VecPackMember for Customer {
    fn get_id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceAddress {
    pub zip: String,
    pub location: String,
    pub street: String,
}

impl Default for InvoiceAddress {
    fn default() -> Self {
        InvoiceAddress {
            zip: String::new(),
            location: String::new(),
            street: String::new(),
        }
    }
}

impl InvoiceAddress {
    pub fn new(zip: String, location: String, street: String) -> Self {
        InvoiceAddress {
            zip,
            location,
            street,
        }
    }
}
