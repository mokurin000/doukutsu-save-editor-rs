use cavestory_save::{
    items::{EquipOpt, Equipment, Inventory, WeaponType},
    strum::IntoEnumIterator,
    GameProfile, Profile, ProfileError,
};

use crate::MainApp;

pub trait ProfileExt {
    fn verify_and_init(&mut self, data: Vec<u8>) -> Result<(), ProfileError>;
    fn update_state(&mut self) -> Option<()>;
    fn detect_equip(&self) -> Option<[bool; 9]>;
    fn count_weapon(&self) -> Option<usize>;
    fn count_inventory(&self) -> Option<usize>;
}

impl ProfileExt for MainApp {
    fn verify_and_init(&mut self, data: Vec<u8>) -> Result<(), ProfileError> {
        match Profile::from_raw_without_length_check(data) {
            Ok(profile) => {
                let game_profile = GameProfile::dump(&profile);
                self.profile = Some((profile, game_profile));
                self.update_state();
                Ok(())
            }
            Err(e) => {
                use rfd::{AsyncMessageDialog, MessageLevel};
                let future = AsyncMessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Load Error")
                    .set_description(&e.to_string())
                    .show();
                #[cfg(target_arch = "wasm32")]
                let _ = poll_promise::Promise::spawn_local(future);

                #[cfg(not(target_arch = "wasm32"))]
                crate::TOKIO_HANDLE.get().unwrap().spawn(future);

                Err(e)
            }
        }
    }

    fn update_state(&mut self) -> Option<()> {
        self.weapon_num = self.count_weapon()?;
        self.inventory_num = self.count_inventory()?;
        self.equip_checked = self.detect_equip()?;
        Some(())
    }

    fn detect_equip(&self) -> Option<[bool; 9]> {
        self.profile
            .as_ref()
            .map(|(_, GameProfile { equipment, .. })| {
                let mut equip_checked: [bool; 9] = Default::default();

                let equip_current = equipment;
                for (i, equip) in Equipment::iter().enumerate() {
                    equip_checked[i] = equip_current.check(equip);
                }

                equip_checked
            })
    }

    fn count_weapon(&self) -> Option<usize> {
        self.profile
            .as_ref()
            .map(|(_, GameProfile { weapon, .. })| {
                weapon
                    .iter()
                    .take_while(|w| w.classification != WeaponType::None)
                    .count()
            })
    }

    fn count_inventory(&self) -> Option<usize> {
        self.profile
            .as_ref()
            .map(|(_, GameProfile { inventory, .. })| {
                inventory
                    .iter()
                    .take_while(|&&i| i != Inventory::None)
                    .count()
            })
    }
}
