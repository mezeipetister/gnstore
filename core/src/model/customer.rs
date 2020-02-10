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
use std::fmt::Debug;
use storaget::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    /// ID for customer
    id: String,
    /// Vector of usernames
    related_users: Vec<String>,
    name: String,
    tax_number: String,
    address: InvoiceAddress,
    phone: String,
    email: String,
    date_created: DateTime<Utc>,
    /// Username who created
    created_by: String,
}

// Implement StorageObject for NotificationContainer
impl StorageObject for Customer {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl Customer {
    pub fn new(
        id: String,
        name: String,
        email: String,
        phone: String,
        tax_number: String,
        zip: String,
        location: String,
        street: String,
        created_by: String,
    ) -> Self {
        Customer {
            id,
            related_users: Vec::new(),
            name,
            tax_number,
            address: InvoiceAddress::new(zip, location, street),
            email,
            phone,
            date_created: Utc::now(),
            created_by,
        }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn has_user(&self) -> bool {
        self.related_users.len() > 0
    }
    pub fn get_users(&self) -> Vec<String> {
        self.related_users.clone()
    }
    pub fn remove_user(&mut self, username: &str) {
        self.related_users.retain(|u| u != username);
    }
    pub fn get_tax_number(&self) -> String {
        self.tax_number.clone()
    }
    pub fn set_tax_number(&mut self, tax_number: String) {
        self.tax_number = tax_number;
    }
    pub fn set_address(&mut self, zip: String, location: String, street: String) {
        self.address.zip = zip;
        self.address.location = location;
        self.address.street = street;
    }
    pub fn get_address(&self) -> (String, String, String) {
        (
            self.address.zip.clone(),
            self.address.location.clone(),
            self.address.street.clone(),
        )
    }
    pub fn get_phone(&self) -> String {
        self.phone.clone()
    }
    pub fn set_phone(&mut self, phone: String) {
        self.phone = phone;
    }
    pub fn get_email(&self) -> String {
        self.email.clone()
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    pub fn get_created_by(&self) -> String {
        self.created_by.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InvoiceAddress {
    zip: String,
    location: String,
    street: String,
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
