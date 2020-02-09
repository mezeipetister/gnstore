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
use nanoid::nanoid;

pub trait Customer {
    /// Get us ID
    fn get_id(&self) -> String;
    /// Get customer name
    fn get_name(&self) -> String;
    /// Set customer name
    fn set_name(&mut self, name: String);
    /// Has user connected to customer
    fn has_user(&self) -> bool;
    /// Get user Some() vector of customer, None if empty
    fn get_users(&self) -> Vec<String>;
    /// Remove connected user by username
    fn remove_user(&mut self, username: &str);
    /// Get tax number
    fn get_tax_number(&self) -> String;
    /// Set tax number
    fn set_tax_number(&mut self, tax_number: String);
    /// Set address
    fn set_address(&mut self, zip: String, location: String, street: String);
    /// Get address
    fn get_address(&self) -> (String, String, String);
    /// Get phone
    fn get_phone(&self) -> String;
    /// Set phone
    fn set_phone(&mut self, phone: String);
    /// Get email
    fn get_email(&self) -> String;
    /// Set email
    fn set_email(&mut self, email: String);
    /// Get date created
    fn get_date_created(&self) -> DateTime<Utc>;
    /// Get created by
    fn get_created_by(&self) -> String;
}

pub fn generate_customer_id() -> String {
    nanoid!(
        10,
        &[
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9',
        ]
    )
}
