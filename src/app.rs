use std::{fs, path::PathBuf};

use cavestory_save::{GameProfile, Profile, ProfileError};

use cavestory_save::items::*;
use cavestory_save::strum::IntoEnumIterator;

use std::sync::mpsc::{Receiver, Sender};

use tap::pipe::Pipe;

pub struct MainApp {
    path: Option<PathBuf>,
    path_sender: Sender<PathBuf>,
    path_receiver: Receiver<PathBuf>,
    profile: Option<(Profile, GameProfile)>,
    weapon_num: usize,
    equip_checked: [bool; 9],
}

impl Default for MainApp {
    fn default() -> Self {
        let (path_sender, path_receiver) = std::sync::mpsc::channel();

        MainApp {
            path: None,
            path_sender,
            path_receiver,
            profile: None,
            weapon_num: 0,
            equip_checked: [false; 9],
        }
    }
}

impl MainApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn verify_and_init(&mut self, data: Vec<u8>) -> Result<(), ProfileError> {
        match Profile::try_from(data) {
            Ok(profile) => {
                let game_profile = GameProfile::dump(&profile);
                self.profile = Some((profile, game_profile));
                self.weapon_num = self.count_weapon().unwrap();
                self.equip_checked = self.detect_equip().unwrap();
                Ok(())
            }
            Err(e) => {
                use rfd::{AsyncMessageDialog, MessageLevel};
                tokio::task::spawn(async move {
                    AsyncMessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Load Error")
                        .set_description(&e.to_string())
                        .show()
                        .await;
                });
                Err(e)
            }
        }
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
}

impl eframe::App for MainApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(path) = self.path_receiver.try_recv() {
            let data: Vec<u8> = fs::read(&path).unwrap();
            if self.verify_and_init(data).is_ok() {
                self.path = Some(path);
            }
        }

        let dragged_path = ctx.input(|i| {
            let dropped_files = &i.raw.hovered_files;

            let file = dropped_files.get(0).map(|df| &df.path).cloned().flatten();
            file
        });

        if let Some(path) = dragged_path {
            let _ = self.path_sender.send(path);
            ctx.input_mut(|i| i.raw.hovered_files.clear());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        (self.path_sender.clone(), ctx.clone())
                            .pipe(|(tx, ctx)| async move {
                                use rfd::AsyncFileDialog;
                                let path = AsyncFileDialog::default()
                                    .add_filter("Profile", &["dat"])
                                    .set_title("Pick your game profile")
                                    .pick_file()
                                    .await;
                                if let Some(path) = path {
                                    let path = path.into();
                                    let _ = tx.send(path);
                                    ctx.request_repaint();
                                }
                            })
                            .pipe(tokio::task::spawn);
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(profile) = &mut self.profile {
                    if ui.button("Undo all").clicked() {
                        profile.1 = GameProfile::dump(&profile.0);
                        let _ = profile;
                        self.weapon_num = self.count_weapon().unwrap();
                        self.equip_checked = self.detect_equip().unwrap();
                    }
                }

                if let Some(profile) = &self.profile {
                    if ui.button("Save").clicked() {
                        let mut modified_profile = profile.0.clone();
                        if let Some(path) = &self.path {
                            profile.1.write(&mut modified_profile);
                            let bytes: Vec<u8> = modified_profile.into();
                            if let Err(e) = fs::write(path, bytes) {
                                use rfd::{AsyncMessageDialog, MessageLevel};
                                tokio::task::spawn(async move {
                                    AsyncMessageDialog::new()
                                        .set_level(MessageLevel::Error)
                                        .set_description(&e.to_string())
                                        .set_title("Error occured on saving!")
                                        .show()
                                        .await;
                                });
                            }
                        }
                    }
                }
            });

            if self.profile.is_none() {
                ui.label("Please load profile.dat");
                ui.label("You can drag it here");
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });

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
mod weapon;
