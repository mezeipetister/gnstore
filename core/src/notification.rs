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
use serde::Serialize;
use std::fmt::Debug;
use storaget::StorageObject;

pub trait NotificationContainer: StorageObject {
    /**
     * Trait type
     */
    type NotificationType: Notification;
    /**
     * Remove notification by id
     */
    fn remove_by_id(&mut self, id: usize) -> AppResult<()>;
    /**
     * Get notification vector
     */
    fn get_notifications(&self) -> &Vec<Self::NotificationType>;
    /**
     * Check if a given ID is available
     */
    fn check_id_is_free(&self, id: usize) -> bool;
    /**
     * Get notification by ID.
     * None if does not exist.
     */
    fn get_by_id(&self, id: usize) -> Option<&Self::NotificationType>;
    /**
     * Add notification to NotificationContainer
     */
    fn add(&mut self, notification: Self::NotificationType);
}

pub trait Notification: Serialize + Debug {
    // Location type
    type Location: Location;
    /**
     * Set location to notification
     */
    fn set_location(&mut self, location: Self::Location);
    /**
     * Set notification to be seen
     * Status change
     */
    fn set_seen(&mut self);
    /**
     * Get Notification ID
     * it only unique per NotficationContainer
     */
    fn get_id(&self) -> usize;
    /**
     * Get DateTime created
     */
    fn get_date_created(&self) -> DateTime<Utc>;
    fn get_is_new(&self) -> bool;
    fn get_subject(&self) -> &str;
    /**
     * returns a generated location url
     * In Angular we should use this url directly
     * for navigation
     */
    fn get_location(&self) -> Option<String>;
}

pub trait Location: Serialize + Debug {
    /**
     * Transform location into String
     */
    fn get_location_url(&self) -> String;
}
