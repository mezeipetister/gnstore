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

pub struct Issue {
    /**
     * ID
     */
    id: String,
    /**
     * Issue title
     */
    title: String,
    /**
     * Issue description
     * TODO: should be markdown capable
     */
    description: String,
    /**
     * Date created, Chrono DateTime<Utc>
     */
    date_created: DateTime<Utc>,
    /**
     * Created by @userid
     */
    created_by: String,
    /**
     * Assigned label list
     */
    labels: Vec<Label>,
    /**
     * Assigned to @userid
     */
    assigned_to: String,
    /**
     * Event list
     */
    events: Vec<Event>,
    /**
     * Number of comments added
     */
    comment_count: usize,
    /**
     * Followed by Vec<@userid: String>
     */
    followed_by: Vec<String>,
    /**
     * Status field
     * true if open, false if closed issue
     */
    is_open: bool,
}

pub struct Label {
    /**
     * e.g.: important
     */
    subject: String,
    /**
     * hex with # or css color code
     * It's important to have a format
     * that is directly processebly by CSS
     * without any modification.
     *
     * e.g.:    #000000
     *          white
     *          green
     */
    text_color: String,
    /**
     * hex with # or css color code
     * It's important to have a format
     * that is directly processebly by CSS
     * without any modification.
     *
     * e.g.:    #000000
     *          white
     *          green
     */
    background_color: String,
}

impl Label {
    pub fn new(subject: String, text_color: String, background_color: String) -> Self {
        Label {
            subject,
            text_color,
            background_color,
        }
    }
}

pub struct Comment {
    /**
     * Comment ID
     * Based on the issue comment_count(er)
     */
    id: usize,
    /**
     * User IDs who liked the comment
     */
    liked: Vec<String>,
    /**
     * Comment text
     * should be markdown ready
     */
    text: String,
}

impl Comment {
    pub fn new(text: String) -> Self {
        Comment {
            // TODO: We need to set ID during the add process
            id: 0,
            liked: Vec::new(),
            text,
        }
    }
}

pub struct Event {
    /**
     * Event created at DateTime<Utc>
     */
    date_created: DateTime<Utc>,
    /**
     * Event created by
     */
    created_by: String,
    /**
     * EventKind stored here
     * This contains all the details
     */
    kind: EventKind,
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

pub enum EventKind {
    /**
     * When new comment arrives
     */
    NewComment(Comment),
    /**
     * New label added
     */
    LabelAdded(Label),
    /**
     * Label removed
     */
    LabelRemoved(Label),
    /**
     * Issue assigned to another user
     */
    AssignedTo(String),
    /**
     * Issue closed
     */
    Closed,
    /**
     * Issue re-opened
     */
    Opened,
}
