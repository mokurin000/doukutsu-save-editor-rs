use cavestory_save::{GameProfile, Profile};

use cavestory_save::items::*;
use cavestory_save::strum::IntoEnumIterator;
use egui::{Context, Ui};

use storage::StorageIO;

use self::utils::ProfileExt;

mod storage;

#[derive(Default)]
pub struct MainApp {
    storage: storage::Storage,
    profile: Option<(Profile, GameProfile)>,
    weapon_num: usize,
    inventory_num: usize,
    equip_checked: [bool; 9],
}

impl MainApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl MainApp {
    fn show_save_button(&self, ui: &mut Ui) {
        if let Some(profile) = &self.profile {
            if ui.button("Save").clicked() {
                let mut modified_profile = profile.0.clone();
                profile.1.write(&mut modified_profile);
                let bytes: Vec<u8> = modified_profile.into();
                self.storage.try_write_data(&bytes);
            }
        }
    }

    fn file_ops(&self, ui: &mut Ui, ctx: &Context) {
        if ui.button("Open").clicked() {
            self.storage.open_dialog(ctx);
        }
        if ui.button("Quit").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close)
        }
    }

    fn draw_editor(&mut self, ctx: &Context) {
        let Some((
            _,
            GameProfile {
                position,
                map,
                music,
                health,
                max_health,
                weapon,
                inventory,
                teleporter: _,
                equipment,
            },
        )) = &mut self.profile
        else {
            return;
        };

        egui::Window::new("Basic").show(ctx, |ui| {
            basic::draw_window(ui, health, max_health, music, map, position);
        });

        egui::Window::new("Equipments").show(ctx, |ui| {
            for (i, equip) in Equipment::iter().enumerate() {
                ui.checkbox(&mut self.equip_checked[i], equip.to_string());
                equipment.switch(equip, self.equip_checked[i]);
            }
        });

        egui::Window::new("Weapons").show(ctx, |ui| {
            weapon::draw_window(ui, &mut self.weapon_num, weapon);
        });

        egui::Window::new("Inventory").show(ctx, |ui| {
            inventory::draw_window(ui, &mut self.inventory_num, inventory);
        });
    }
}

impl eframe::App for MainApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(data) = self.storage.try_read_data() {
            let _ = self.verify_and_init(data);
        }

        self.storage.drag_handle(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    self.file_ops(ui, &ctx);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(profile) = &mut self.profile {
                    if ui.button("Undo all").clicked() {
                        profile.1 = GameProfile::dump(&profile.0);
                        self.update_state();
                    }
                }
                self.show_save_button(ui);
            });

            if self.profile.is_none() {
                ui.label("Please load profile.dat");
                ui.label("You can drag it here");
            } else {
                self.draw_editor(ctx);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        [12., 12., 12., 180.]

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

mod basic;
mod inventory;
mod weapon;

mod utils;
