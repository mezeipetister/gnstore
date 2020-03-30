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

use crate::issue::*;
pub use crate::model::version::issue::comment::v1::Comment;
pub use crate::model::version::issue::event::v1::{Event, EventKind};
pub use crate::model::version::issue::label::v1::Label;
pub use crate::model::version::issue::v1::Issue;
use crate::prelude::AppResult;
use crate::Error;
use chrono::prelude::*;

impl Issue {
    pub fn new(title: String, description: String, created_by: String) -> Self {
        Issue {
            // TODO: Change it to have a counter and use usize instead!
            id: generate_issue_id(),
            title,
            description,
            date_created: Utc::now(),
            created_by: created_by.clone(),
            labels: Vec::new(),
            assigned_to: created_by,
            events: Vec::new(),
            comment_count: 0,
            followed_by: Vec::new(),
            is_open: true,
        }
    }
    /**
     * Get title
     */
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    /**
     * Set title
     */
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    /**
     * Get description
     */
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    /**
     * Get userid created by
     */
    pub fn get_created_by(&self) -> String {
        self.created_by.clone()
    }
    /**
     * Get date created
     * DateTime<Utc>
     */
    pub fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    /**
     * Get related labels
     */
    pub fn get_labels(&self) -> Vec<Label> {
        self.labels.clone()
    }
    /**
     * Add label to issue
     */
    pub fn add_label(&mut self, label: Label, created_by: String) {
        // Check if the label is already in the label list
        if let Some(_) = self.labels.iter().position(|l| *l == label) {
            return;
        }
        // If the label is not in the vector,
        // then we add it
        self.labels.push(label.clone());
        // And now add the label to the event list
        self.events
            .push(Event::new(created_by, EventKind::LabelAdded(label)));
    }
    /**
     * Remove a given label is if it's in the list
     */
    pub fn remove_label(&mut self, label: Label, created_by: String) {
        // Check if the label is in the label list
        if let Some(_) = self.labels.iter().position(|l| *l == label) {
            // First remove label if match
            self.labels.retain(|l| *l != label.clone());
            // Then create an event
            self.events
                .push(Event::new(created_by, EventKind::LabelRemoved(label)));
        }
    }
    /**
     * Get assigned to value
     * returns @user: String
     */
    pub fn get_assigned_to(&self) -> String {
        self.assigned_to.clone()
    }
    /**
     * Set new assigned value to @user: String
     */
    pub fn set_assigned_to(&mut self, user: String, created_by: String) {
        // If the user already assigned, then
        // return without any change
        if self.assigned_to == user {
            return;
        }
        // Set assigned_to value
        self.assigned_to = user.clone();
        // Create an event from it
        self.events
            .push(Event::new(created_by, EventKind::AssignedTo(user)));
    }
    /**
     * Get event vector
     */
    pub fn get_events(&self) -> Vec<Event> {
        self.events.clone()
    }
    /**
     * Get comment count: usize
     */
    pub fn get_comment_count(&self) -> usize {
        self.comment_count
    }
    /**
     * Create a comment and then add it
     * as a NewComment event.
     */
    pub fn add_comment(&mut self, text: String, created_by: String) {
        // Increase comment_count by one
        self.comment_count += 1;
        // Push comment to events
        self.events.push(Event::new(
            created_by,
            EventKind::NewComment(Comment::new(self.comment_count, text)),
        ));
    }
    /**
     * Looking for comment by a given ID
     * if we find it, then set like by user_id: String
     */
    pub fn like_comment(&mut self, comment_id: usize, user_id: String) -> AppResult<()> {
        for event in &mut self.events {
            if let EventKind::NewComment(comment) = &mut event.kind {
                if comment.get_id() == comment_id {
                    comment.like(user_id);
                    return Ok(());
                }
            }
        }
        Err(Error::BadRequest(
            "A megadott comment ID nem létezik".to_owned(),
        ))
    }
    /**
     * Looking for comment by a given ID
     * if we find it, then set like by user_id: String
     */
    pub fn dislike_comment(&mut self, comment_id: usize, user_id: String) -> AppResult<()> {
        for event in &mut self.events {
            if let EventKind::NewComment(comment) = &mut event.kind {
                if comment.get_id() == comment_id {
                    comment.unlike(user_id);
                    return Ok(());
                }
            }
        }
        Err(Error::BadRequest(
            "A megadott comment ID nem létezik".to_owned(),
        ))
    }
    /**
     * Add user_id to the followed_by list
     * if the user is already not in the list
     */
    pub fn follow(&mut self, user_id: String) {
        // Check if user is not in the list
        if let None = self.followed_by.iter().position(|u| *u == user_id) {
            self.followed_by.push(user_id);
        }
    }
    /**
     * Unfollow issue by user
     * Remove user_id if it represents in the list
     */
    pub fn unfollow(&mut self, user_id: String) {
        self.followed_by.retain(|u| *u != user_id);
    }
    /**
     * Get user vector of followed_by
     */
    pub fn get_followed_by(&self) -> Vec<String> {
        self.followed_by.clone()
    }
    /**
     * Set is_open status to true
     * and create an event about it
     */
    pub fn open_issue(&mut self, created_by: String) {
        self.is_open = true;
        self.events.push(Event::new(created_by, EventKind::Opened));
    }
    /**
     * Set is_open status to false
     * and create and event about it
     */
    pub fn close_issue(&mut self, created_by: String) {
        self.is_open = false;
        self.events.push(Event::new(created_by, EventKind::Closed));
    }
    /**
     * Get is_open status: bool
     */
    pub fn get_is_open(&self) -> bool {
        self.is_open
    }
}

impl Label {
    pub fn new(subject: String, text_color: String, background_color: String) -> Self {
        Label {
            subject,
            text_color,
            background_color,
        }
    }
    pub fn get_subject(&self) -> String {
        self.subject.clone()
    }
    pub fn get_text_color(&self) -> String {
        self.text_color.clone()
    }
    pub fn get_background_color(&self) -> String {
        self.background_color.clone()
    }
}

impl Comment {
    /**
     * Get ID: usize
     */
    pub fn get_id(&self) -> usize {
        self.id
    }
    /**
     * Like comment
     * Add user_id if it's not in the list
     */
    pub fn like(&mut self, user_id: String) {
        if let None = self.liked.iter().position(|c| *c == user_id) {
            self.liked.push(user_id);
        }
    }
    /**
     * Remove userid if its in the list
     */
    pub fn unlike(&mut self, user_id: String) {
        self.liked.retain(|c| *c != user_id);
    }
}
