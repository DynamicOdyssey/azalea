#![doc = include_str!("../README.md")]
#![feature(min_specialization)]

pub mod components;
pub mod default_components;
pub mod item;
pub mod operations;
mod slot;

use std::ops::{Deref, DerefMut, RangeInclusive};

use azalea_inventory_macros::declare_menus;
pub use slot::{DataComponentPatch, ItemStack, ItemStackData};

// TODO: remove this here and in azalea-inventory-macros when rust makes
// Default be implemented for all array sizes
// https://github.com/rust-lang/rust/issues/61415

/// A fixed-size list of [`ItemStack`]s.
#[derive(Clone, Debug)]
pub struct SlotList<const N: usize>([ItemStack; N]);
impl<const N: usize> Deref for SlotList<N> {
    type Target = [ItemStack; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize> DerefMut for SlotList<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<const N: usize> Default for SlotList<N> {
    fn default() -> Self {
        SlotList([(); N].map(|_| ItemStack::Empty))
    }
}
impl<const N: usize> SlotList<N> {
    pub fn new(items: [ItemStack; N]) -> Self {
        SlotList(items)
    }
}

impl Menu {
    /// Get the [`Player`] from this [`Menu`].
    ///
    /// # Panics
    ///
    /// Will panic if the menu isn't `Menu::Player`.
    pub fn as_player(&self) -> &Player {
        self.try_as_player()
            .expect("Called `Menu::as_player` on a menu that wasn't `Player`.")
    }
    /// Get the [`Player`] from this [`Menu`], or returns `None` if the menu
    /// isn't a player menu.
    pub fn try_as_player(&self) -> Option<&Player> {
        if let Menu::Player(player) = &self {
            Some(player)
        } else {
            None
        }
    }

    /// Same as [`Menu::as_player`], but returns a mutable reference to the
    /// [`Player`].
    ///
    /// # Panics
    ///
    /// Will panic if the menu isn't `Menu::Player`.
    pub fn as_player_mut(&mut self) -> &mut Player {
        self.try_as_player_mut()
            .expect("Called `Menu::as_player_mut` on a menu that wasn't `Player`.")
    }
    /// Same as [`Menu::try_as_player`], but returns a mutable reference to the
    /// [`Player`].
    pub fn try_as_player_mut(&mut self) -> Option<&mut Player> {
        if let Menu::Player(player) = self {
            Some(player)
        } else {
            None
        }
    }
}

// the player inventory part is always the last 36 slots (except in the Player
// menu), so we don't have to explicitly specify it

// Client {
//     ...
//     pub menu: Menu,
//     pub inventory: Arc<[Slot; 36]>
// }

// Generate a `struct Player`, `enum Menu`, and `impl Menu`.
// a "player" field gets implicitly added with the player inventory

declare_menus! {
    Player {
        craft_result: 1,
        craft: 4,
        armor: 4,
        inventory: 36,
        offhand: 1,
    },
    Generic9x1 {
        contents: 9,
    },
    Generic9x2 {
        contents: 18,
    },
    Generic9x3 {
        contents: 27,
    },
    Generic9x4 {
        contents: 36,
    },
    Generic9x5 {
        contents: 45,
    },
    Generic9x6 {
        contents: 54,
    },
    Generic3x3 {
        contents: 9,
    },
    Crafter3x3 {
        contents: 9,
    },
    Anvil {
        first: 1,
        second: 1,
        result: 1,
    },
    Beacon {
        payment: 1,
    },
    BlastFurnace {
        ingredient: 1,
        fuel: 1,
        result: 1,
    },
    BrewingStand {
        bottles: 3,
        ingredient: 1,
        fuel: 1,
    },
    Crafting {
        result: 1,
        grid: 9,
    },
    Enchantment {
        item: 1,
        lapis: 1,
    },
    Furnace {
        ingredient: 1,
        fuel: 1,
        result: 1,
    },
    Grindstone {
        input: 1,
        additional: 1,
        result: 1,
    },
    Hopper {
        contents: 5,
    },
    Lectern {
        book: 1,
    },
    Loom {
        banner: 1,
        dye: 1,
        pattern: 1,
        result: 1,
    },
    Merchant {
        payments: 2,
        result: 1,
    },
    ShulkerBox {
        contents: 27,
    },
    Smithing {
        template: 1,
        base: 1,
        additional: 1,
        result: 1,
    },
    Smoker {
        ingredient: 1,
        fuel: 1,
        result: 1,
    },
    CartographyTable {
        map: 1,
        additional: 1,
        result: 1,
    },
    Stonecutter {
        input: 1,
        result: 1,
    },
    // A horse, donkey, mule or llama's inventory. The server opens it with
    // ClientboundMountScreenOpen, not a MenuKind; see `Menu::mount`.
    Mount {
        saddle: 1,
        armor: 1,
    },
    MountChest3 {
        saddle: 1,
        armor: 1,
        contents: 3,
    },
    MountChest6 {
        saddle: 1,
        armor: 1,
        contents: 6,
    },
    MountChest9 {
        saddle: 1,
        armor: 1,
        contents: 9,
    },
    MountChest12 {
        saddle: 1,
        armor: 1,
        contents: 12,
    },
    MountChest15 {
        saddle: 1,
        armor: 1,
        contents: 15,
    },
}

impl Menu {
    /// The menu for a mount's inventory (`ClientboundMountScreenOpen`):
    /// saddle, body armor, then `columns * 3` chest slots (a donkey or mule
    /// with a chest has 5 columns), then the player's 36 slots.
    pub fn mount(columns: u32) -> Self {
        match columns {
            0 => Menu::Mount {
                saddle: Default::default(),
                armor: Default::default(),
                player: Default::default(),
            },
            1 => Menu::MountChest3 {
                saddle: Default::default(),
                armor: Default::default(),
                contents: Default::default(),
                player: Default::default(),
            },
            2 => Menu::MountChest6 {
                saddle: Default::default(),
                armor: Default::default(),
                contents: Default::default(),
                player: Default::default(),
            },
            3 => Menu::MountChest9 {
                saddle: Default::default(),
                armor: Default::default(),
                contents: Default::default(),
                player: Default::default(),
            },
            4 => Menu::MountChest12 {
                saddle: Default::default(),
                armor: Default::default(),
                contents: Default::default(),
                player: Default::default(),
            },
            _ => Menu::MountChest15 {
                saddle: Default::default(),
                armor: Default::default(),
                contents: Default::default(),
                player: Default::default(),
            },
        }
    }
}

#[cfg(test)]
mod mount_tests {
    use super::*;

    #[test]
    fn donkey_with_a_chest_lays_out_like_the_game() {
        // HorseInventoryMenu: saddle 0, body armor 1, chest 2.., then the
        // player.
        let m = Menu::mount(5);
        assert_eq!(m.len(), 2 + 15 + 36);
        assert_eq!(m.contents().len(), 17);
        assert_eq!(m.player_slots_range(), 17..=52);
        assert_eq!(m.hotbar_slots_range(), 44..=52);
        assert_eq!(Menu::MOUNT_CHEST15_CONTENTS_SLOTS, 2..=16);
        assert!(matches!(
            m.location_for_slot(0),
            Some(MenuLocation::MountChest15(MountChest15MenuLocation::Saddle))
        ));
        assert!(matches!(
            m.location_for_slot(2),
            Some(MenuLocation::MountChest15(
                MountChest15MenuLocation::Contents
            ))
        ));
        // No chest (a horse): just saddle and armor.
        assert_eq!(Menu::mount(0).player_slots_range(), 2..=37);
        assert_eq!(Menu::mount(3).len(), 2 + 9 + 36);
    }
}
