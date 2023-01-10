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
use std::collections::HashMap;

use crate::utils::http::CardSetMapType;

/// Type contained in a processed cardinfo binary file.
pub type CardinfoMetaType = HashMap<u32, Card>;

/// External [URL](https://db.ygoprodeck.com/api/v7/cardinfo.php) to the card data.
pub const EXT_URL: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";

/// Representation of one card set of a card from the YGOPRODECK API.
#[derive(Serialize, Deserialize, Debug)]
pub struct YGOPDCardSet {
    pub set_name: String,
    pub set_code: String,
    pub set_rarity: String,
}

/// Representation of one card from the YGOPRODECK API.
#[derive(Serialize, Deserialize, Debug)]
pub struct YGOPDCard {
    pub id: u32,
    pub name: String,
    pub r#type: String,
    pub desc: String,
    pub atk: Option<u32>,
    pub def: Option<u32>,
    pub level: Option<u8>,
    pub race: String,
    pub attribute: Option<String>,
    pub archetype: Option<String>,
    pub scale: Option<u8>,
    pub linkval: Option<u8>,
    pub card_sets: Option<Vec<YGOPDCardSet>>,
}

/// Representation of the data from the YGOPRODECK API.
#[derive(Serialize, Deserialize, Debug)]
pub struct YGOPDData {
    pub data: Vec<YGOPDCard>,
}

/// Representation of processed card data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub card_type: String,
    pub description: String,
    pub atk: Option<u32>,
    pub def: Option<u32>,
    pub level: Option<u8>,
    pub r#type: String,
    pub attribute: Option<String>,
    pub archetype: Option<String>,
    pub pend_scale: Option<u8>,
    pub link_rating: Option<u8>,
}

/// Returns a processed cardinfo map.
///
/// # Arguments
///
/// * `cardinfo` – Slice containing raw cardinfo json data.
///
/// * `card_set_map` – HashMap to be populated with card ids.
pub fn parse(cardinfo: &str, card_set_map: &mut CardSetMapType) -> CardinfoMetaType {
    let mut cardinfo_map: CardinfoMetaType = HashMap::new();

    // Iterate of cards in data
    for card in serde_json::from_str::<YGOPDData>(cardinfo).unwrap().data {
        cardinfo_map.insert(
            card.id,
            Card {
                id: card.id,
                name: card.name,
                card_type: card.r#type,
                description: card.desc,
                atk: card.atk,
                def: card.def,
                level: card.level,
                attribute: card.attribute,
                r#type: card.race,
                archetype: card.archetype,
                pend_scale: card.scale,
                link_rating: card.linkval,
            },
        );

        // Extract card_sets and put them into card_set_map
        if card.card_sets.is_some() {
            for card_set in card.card_sets.unwrap() {
                let val = card_set_map.get_mut(card_set.set_name.as_str());

                // Check if there already is a Vec at val
                if val.is_some() {
                    if !val.as_ref().unwrap().contains(&card.id) {
                        val.unwrap().push(card.id);
                    }
                } else {
                    card_set_map.insert(card_set.set_name, vec![card.id]);
                }
            }
        }
    }

    return cardinfo_map;
}
