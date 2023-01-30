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

use std::collections::{HashMap, VecDeque};
use std::fs;

use bincode::{
    config::{BigEndian, Configuration, Fixint},
    serde::decode_from_slice as decode,
    serde::encode_to_vec as encode,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::user_data::{
    Action,
    Card,
    Change,
    CollectionBuilder,
    MetaData,
};
use crate::utils::PATHS;

/// Bincode configuration for all collections.
static BINCODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .write_fixed_array_length();

pub static LAST_CHANGED_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Collection {
    pub meta_data: MetaData,
    pub cards: HashMap<Card, u8>,
    pub changes: VecDeque<Change>,
    pub tags: HashMap<String, Vec<Card>>,
    pub actions: Vec<Action>,
}

impl Collection {
    /// Constructs a [`CollectionBuilder`].
    pub fn builder() -> CollectionBuilder {
        CollectionBuilder::new()
    }

    /// Returns the names of all locally saved collections.
    pub fn get_names() -> Vec<String> {
        if let Ok(read_dir) = PATHS.user_paths.collections.read_dir() {
            read_dir
                .map(|path| {
                    path.expect(&format!("Failed to read path."))
                        .file_name()
                        .into_string()
                        .expect("Failed to get file name.")
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets a collection from its file name.
    ///
    /// # Arguments
    ///
    /// * `name` – Name of the collection.
    pub fn from_name(name: &String) -> Self {
        decode(
            &fs::read(&PATHS.user_paths.collections.join(name))
                .expect("Failed to read collection."),
            BINCODE_CONFIG,
        )
        .expect("Failed to decode collection.")
        .0
    }

    /// Saves a collection to a file.
    ///
    /// # Arguments
    ///
    /// * `name` – Name of the collection.
    pub fn save(&mut self, name: &String) {
        self.meta_data.last_changed = format!("{}", Utc::now().format(LAST_CHANGED_FORMAT));
        fs::write(
            &PATHS.user_paths.collections.join(name),
            encode(self, BINCODE_CONFIG).unwrap(),
        )
        .expect("Failed to save collection.");
    }

    /// Gets only a collection's [`MetaData`].
    ///
    /// # Arguments
    ///
    /// * `name` – Name of the collection.
    pub fn get_metadata_from(name: &String) -> MetaData {
        decode(
            &fs::read(&PATHS.user_paths.collections.join(name))
                .expect("Failed to read collection."),
            BINCODE_CONFIG,
        )
        .expect("Failed to decode collection.")
        .0
    }

    /// Adds a new `Change` and applies it to [`cards`][`Collection::cards`].
    pub fn add_change(&mut self, change: Change) {
        match &change {
            Change::Add(content) => self.add_cards(&content.cards),
            Change::Remove(content) => self.remove_cards(&content.cards),
            _ => return,
        }

        self.changes.push_front(change);
    }

    /// Removes the most recent `Change` and applies the inverse action to
    /// [`cards`][`Collection::cards`].
    pub fn undo_change(&mut self) {
        let change = self.changes.pop_front().unwrap();

        match change {
            Change::Add(content) => self.remove_cards(&content.cards),
            Change::Remove(content) => self.add_cards(&content.cards),
            _ => return,
        }
    }

    /// # Arguments
    ///
    /// * `cards` – Reference to the cards to add.
    fn add_cards(&mut self, cards: &Vec<Card>) {
        for card in cards.iter() {
            if let Some(quantity) = self.cards.get(card) {
                self.cards.insert(card.clone(), quantity + 1);
            } else {
                self.cards.insert(card.clone(), 1);
            }
        }
    }

    /// # Arguments
    ///
    /// * `cards` – Reference to the cards to remove.
    fn remove_cards(&mut self, cards: &Vec<Card>) {
        for card in cards.iter() {
            if let Some(quantity) = self.cards.get(card) {
                if quantity < &1 {
                    self.cards.remove(card);
                } else {
                    self.cards.insert(card.clone(), quantity - 1);
                }
            }
        }
    }
}
