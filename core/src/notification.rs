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
use serde::Serialize;
use std::fmt::Debug;

pub trait Notifications<T>
where
    T: Notification + Serialize + Debug,
{
    /// Remove notification by id
    fn remove_by_id(&mut self, id: usize) -> AppResult<()>;
    /// Add new notification
    fn add(&mut self, notification: T) -> AppResult<()>;
    /// Get notification vector
    fn get_notifications(&self) -> Vec<T>;
}

pub trait Notification {
    /// Set notification to be seen
    /// Status change
    fn set_seen(&mut self);
}
