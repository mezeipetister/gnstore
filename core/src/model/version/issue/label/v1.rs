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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Label {
    /**
     * e.g.: important
     */
    pub subject: String,
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
    pub text_color: String,
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
    pub background_color: String,
}
