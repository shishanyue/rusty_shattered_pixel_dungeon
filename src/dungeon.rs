use std::collections::HashMap;

use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum LimitedDropsKind {
    // Limited world drops
    StrengthPotions,
    UpgradeScrolls,
    ArcaneStyli,
    EnchStone,
    IntStone,
    TrinketCata,
    LabRoom, // Actually a room, but logic is the same

    // Health potion sources
    // Enemies
    SwarmHp,
    NecroHp,
    BatHp,
    WarlockHp,

    // Demon spawners are already limited in their spawnrate, no need to limit their health drop
    // Alchemy
    CookingHp,
    BlandfruitSeed,

    // Other limited enemy drops
    SlimeWep,
    SkeleWep,
    TheifMisc,
    GuardArm,
    ShamanWand,
    Dm200Equip,

    // Containers
    VelvetPouch,

    ScrollHolder,

    PotionBandolier,

    MagicalHolster,

    // Lore documents
    LoreSewers,

    LorePrison,

    LoreCaves,

    LoreCity,

    LoreHalls,
}

pub struct LimitedDrops {
    pub counts: HashMap<LimitedDropsKind, i32>,
}
impl LimitedDrops {
    //for items which can only be dropped once, should directly access count otherwise.
    pub fn dropped(&self, kind: LimitedDropsKind) -> bool {
        *self.counts.get(&kind).unwrap() != 0
    }

    pub fn drop(&mut self, kind: LimitedDropsKind) {
        *self.counts.get_mut(&kind).unwrap() = 1;
    }

    pub fn reset(&mut self) {
        for (_, value) in self.counts.iter_mut() {
            *value = 0
        }
    }
    pub fn store(&mut self, kind: LimitedDropsKind, value: i32) {}
    pub fn restore(&mut self, kind: LimitedDropsKind, value: i32) {}
}

impl Default for LimitedDrops {
    fn default() -> Self {
        let mut counts = HashMap::new();
        for kind in LimitedDropsKind::iter() {
            counts.insert(kind, 0);
        }
        Self { counts }
    }
}

#[derive(Component, Default)]
pub struct Dungeon {
    pub limited_drops: LimitedDrops,
}

impl Dungeon {
    pub fn new() {
        todo!()
    }
}
