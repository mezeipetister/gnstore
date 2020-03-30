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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    /**
     * Comment ID
     * Based on the issue comment_count(er)
     */
    pub id: usize,
    /**
     * User IDs who liked the comment
     */
    pub liked: Vec<String>,
    /**
     * Comment text
     * should be markdown ready
     */
    pub text: String,
}

impl Comment {
    pub fn new(id: usize, text: String) -> Self {
        Comment {
            // TODO: We need to set ID during the add process
            id,
            liked: Vec::new(),
            text,
        }
    }
}
