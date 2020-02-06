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

use crate::error::Error;
use crate::notification::*;
use crate::prelude::AppResult;
use chrono::prelude::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use storaget::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Location {
    /**
     * When we have no location provided
     */
    None,
    /**
     * Raw link
     * Client should use it as a link directly
     */
    Raw(String),
    /**
     * Issue location
     * @id: usize => issue ID
     * @section: String => Section tag #
     */
    Issue { id: usize, section: String },
}

impl Location {
    /**
     * Transform location into String
     */
    pub fn get_location_url(&self) -> String {
        match self {
            Location::None => "".to_owned(),
            Location::Raw(url) => url.to_owned(),
            Location::Issue { id, section } => format!("/a/issue/{}/{}", id, section),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationContainerV1<T>
where
    T: Notification,
{
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
     * Using this counter for notification ID
     * generation.
     */
    notification_counter: usize,
    /**
     * Notification holder
     * We use vector, but as we store in serialized
     * the vector item order is persistent.
     * TODO: Verify it
     */
    notifications: Vec<Box<T>>,
}

impl<T> NotificationContainerV1<T>
where
    T: Notification,
{
    pub fn new(id: String) -> Self {
        NotificationContainerV1 {
            id,
            notification_counter: 0,
            notifications: Vec::new(),
        }
    }
}

impl<T> NotificationContainer<T> for NotificationContainerV1<T>
where
    T: Notification,
{
    /**
     * Remove notification by ID
     */
    fn remove_by_id(&mut self, id: usize) -> AppResult<()> {
        match self.notifications.iter().position(|x| x.get_id() == id) {
            // If we have a (first) poistion
            Some(index) => {
                let _ = self.notifications.remove(index);
                return Ok(());
            }
            // If there is no notification with the given ID
            None => Err(Error::BadRequest(
                "A kért ID-val nem létezik értesítés.".to_owned(),
            )),
        }
    }
    /**
     * Get notifications vector
     */
    fn get_notifications(&self) -> &Vec<Box<T>> {
        &self.notifications
    }
    /**
     * Check id wether free
     */
    fn check_id_is_free(&self, id: usize) -> bool {
        if self
            .notifications
            .iter()
            .position(|x| x.get_id() == id)
            .is_some()
        {
            return true;
        }
        false
    }
    /**
     * Get notification by id
     */
    fn get_by_id(&self, id: usize) -> Option<&T> {
        match self.notifications.iter().position(|x| x.get_id() == id) {
            Some(index) => match self.notifications.get(index) {
                Some(result) => Some(result),
                None => None,
            },
            None => None,
        }
    }
    /**
     * Add new notification to notification container
     */
    fn add(&mut self, notification: T) {
        // Increment counter
        self.notification_counter += 1;
        // Create new notification
        // let new_notification: Box<dyn Notification> = Box::new(NotificationV1 {
        //     id: self.notification_counter,
        //     date_created: Utc::now(),
        //     is_new: true,
        //     subject,
        //     location,
        // });
        self.notifications.push(Box::new(notification));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationV1 {
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
    location: Location,
}

impl NotificationV1 {
    /**
     * Add location to Notification
     */
    pub fn add_location(&mut self, location: Location) {
        self.location = location;
    }
}

impl Notification for NotificationV1 {
    /**
     * Set seen to false
     */
    fn set_seen(&mut self) {
        self.is_new = false;
    }
    /**
     * Get Notification ID
     */
    fn get_id(&self) -> usize {
        self.id
    }
    /**
     * Transform location data into String
     */
    fn get_location_url(&self) -> String {
        self.location.get_location_url()
    }
}
