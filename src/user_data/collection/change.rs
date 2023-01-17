/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use serde::{Deserialize, Serialize};

use super::Card;

/// Content that is applied by a [`Change`].
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChangeContent {
    pub cards: Vec<Card>,
    pub date: String,
    pub round: Option<u16>,
}

/// A modification that is applied to a [`Collection`].
#[derive(Serialize, Deserialize, Default, Debug)]
pub enum Change {
    #[default]
    None,
    Add(ChangeContent),
    Remove(ChangeContent),
}

impl ChangeContent {
    /// Creates content used by a change.
    ///
    /// # Arguments
    ///
    /// * `cards` – Cards that are to be changed.
    /// * `date` – Date when the change was executed.
    /// * `round` – Optional. The draft round the change was executed in.
    pub fn new(cards: Vec<Card>, date: String, round: Option<u16>) -> Self {
        Self {
            cards: cards,
            date: date,
            round: round,
        }
    }
}
