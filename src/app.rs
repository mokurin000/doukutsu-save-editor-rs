use cavestory_save::{GameProfile, Profile};

use cavestory_save::items::*;
use cavestory_save::strum::IntoEnumIterator;
use egui::{Context, Panel, Ui};

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

    fn file_ops(&mut self, ui: &mut Ui) {
        if ui.button("Open").clicked() {
            self.storage.open_dialog();
        }
        if let Some((_, gameprofile)) = &mut self.profile {
            if ui.button("Enable all teleporters").clicked() {
                MainApp::enable_all_teleporters(gameprofile);
            }
            if ui.button("Clear").clicked() {
                self.profile.take();
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Quit").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close)
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(data) = self.storage.try_read_data() {
            let _ = self.verify_and_init(data);
        }

        self.storage.drag_handle(ui.ctx());

        Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.set_width(80.0);
                    self.file_ops(ui);
                });
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
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
                self.draw_editor(ui.ctx());
            }

            #[cfg(target_arch = "wasm32")]
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("To have a smoother experience, you can download ");
                    ui.hyperlink_to(
                        "native binaries",
                        "https://github.com/mokurin000/doukutsu-save-editor-rs/releases/latest",
                    );
                    ui.label(" on github.");
                });
            });
        });
    }
}

mod basic;
mod inventory;
mod weapon;

mod utils;
