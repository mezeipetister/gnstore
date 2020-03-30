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

use crate::model::version::issue::comment::v1::Comment;
use crate::model::version::issue::label::v1::Label;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "body")]
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
