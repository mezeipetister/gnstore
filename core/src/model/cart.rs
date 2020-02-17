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

pub struct Cart {
    id: String,
    created_by: String,
    date_created: DateTime<Utc>,
    is_closed: bool,
    item_count: usize,
    cart_items: Vec<CartItem>,
    total_net_price: usize,
    total_gross_price: usize,
    invoice: Option<String>,
}

pub struct CartItem {
    product_id: String,
    product_name: String,
    piece: usize,
    net_price: usize,
    vat: usize,
}
