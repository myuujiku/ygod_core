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

mod battle_pack_settings;
mod choice_settings;

use serde::{Deserialize, Serialize};

pub use battle_pack_settings::BattlePackSettings;
pub use choice_settings::ChoiceSettings;

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum DraftBox {
    #[default]
    None,
    BattlePackDraft(BattlePackSettings),
    ChoiceDraft(ChoiceSettings),
    // StandardDraft(StandardSettings),
}

impl DraftBox {
    pub fn is_battle_pack_draft(&self) -> bool {
        match self {
            DraftBox::BattlePackDraft(_) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DraftBoxMeta {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum SetRotation {
    #[default]
    Disabled,
    Enabled(usize),
}
