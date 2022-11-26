#[cfg(not(target_arch = "wasm32"))]
use std::{fs, path::PathBuf};

use cavestory_save::{GameProfile, Profile};

use cavestory_save::items::*;
use strum::IntoEnumIterator;

use egui::{DragValue, Slider};

pub struct MainApp {
    #[cfg(target_arch = "wasm32")]
    input: String,
    #[cfg(not(target_arch = "wasm32"))]
    path: Option<PathBuf>,
    profile: Option<GameProfile>,
    raw_profile: Option<Profile>,
    weapon_num: usize,
}

impl Default for MainApp {
    #[cfg(not(target_arch = "wasm32"))]
    fn default() -> Self {
        Self {
            path: None,
            profile: None,
            raw_profile: None,
            weapon_num: 0,
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn default() -> Self {
        Self {
            input: String::new(),
            profile: None,
            raw_profile: None,
            weapon_num: 0,
        }
    }
}

impl MainApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn verify_and_init(&mut self, data: Profile) -> bool {
        if data.verify() {
            self.profile = Some(GameProfile::dump(&data));
            self.raw_profile = Some(data);
            self.weapon_num = self.count_weapon().unwrap();
            true
        } else {
            use rfd::{MessageDialog, MessageLevel};
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Load Error")
                .set_description("Profile.dat head not equal to \"Do041220\"")
                .show();
            false
        }
    }

    fn count_weapon(&self) -> Option<usize> {
        self.profile.and_then(|p| {
            Some(
                p.weapon
                    .iter()
                    .take_while(|w| w.classification != WeaponType::None)
                    .count(),
            )
        })
    }
}

impl eframe::App for MainApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            use rfd::FileDialog;
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let path = FileDialog::default()
                            .add_filter("Profile", &["dat"])
                            .set_title("Pick your game profile")
                            .pick_file();
                        if let Some(path) = path {
                            let data: Profile = fs::read(&path).unwrap().into();
                            if self.verify_and_init(data) {
                                self.path = Some(path);
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            #[cfg(target_arch = "wasm32")]
            {
                use rfd::{MessageDialog, MessageLevel};

                ui.label("Paste profile.dat encoded in base64 here:");

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input);
                    if ui.button("Load Profile").clicked() {
                        self.input.retain(|c| !c.is_ascii_whitespace());
                        if let Ok(bytes) = base64::decode(&self.input) {
                            if self.verify_and_init(bytes.into()) {
                                self.input.clear();
                            }
                        } else {
                            MessageDialog::new()
                                .set_description("Invalid base64 code!")
                                .set_level(MessageLevel::Error)
                                .show();
                        }
                    }
                });
            }

            if let Some(profile) = &mut self.profile {
                egui::Window::new("Basic").show(ctx, |ui| {
                    ui.add(DragValue::new(&mut profile.health).prefix("heal: "));
                    ui.add(DragValue::new(&mut profile.max_health).prefix("max heal: "));

                    ui.label("BGM");
                    egui::ComboBox::new("background_music", "")
                        .selected_text(profile.music.to_string())
                        .width(200.)
                        .show_ui(ui, |ui| {
                            for bg_music in Song::iter() {
                                ui.selectable_value(
                                    &mut profile.music,
                                    bg_music,
                                    bg_music.to_string(),
                                );
                            }
                        });

                    ui.label("Map");
                    egui::ComboBox::new("map", "")
                        .selected_text(profile.map.to_string())
                        .width(200.)
                        .show_ui(ui, |ui| {
                            for map in Map::iter() {
                                ui.selectable_value(&mut profile.map, map, map.to_string());
                            }
                        });

                    ui.label("Position");
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut profile.position.x).prefix("x: "));
                        ui.add(DragValue::new(&mut profile.position.y).prefix("y: "));
                    });
                });

                egui::Window::new("Weapons").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // do not set the 8th weapon, you may go into issue.
                        if ui.button(" + ").clicked() && self.weapon_num < 7 {
                            self.weapon_num += 1
                        }
                        if ui.button(" - ").clicked() && self.weapon_num > 0 {
                            self.weapon_num -= 1;
                            profile.weapon[self.weapon_num] = Weapon::default();
                        }
                    });

                    ui.separator();

                    for (chunk_i, chunk) in
                        profile.weapon[..self.weapon_num].chunks_mut(3).enumerate()
                    {
                        ui.horizontal(|ui| {
                            for (i, weapon) in chunk.iter_mut().enumerate() {
                                ui.vertical(|ui| {
                                    egui::ComboBox::new(
                                        format!("weapontype-box-{}", chunk_i * 3 + i),
                                        "",
                                    )
                                    .width(150.)
                                    .selected_text(weapon.classification.to_string())
                                    .show_ui(ui, |ui| {
                                        for model in WeaponType::iter() {
                                            ui.selectable_value(
                                                &mut weapon.classification,
                                                model,
                                                model.to_string(),
                                            );
                                        }
                                    });
                                    if weapon.classification != WeaponType::None {
                                        ui.label("level");
                                        ui.add(Slider::new(&mut weapon.level, 0..=3));

                                        ui.add(DragValue::new(&mut weapon.ammo).prefix("ammo: "));
                                        ui.add(
                                            DragValue::new(&mut weapon.max_ammo)
                                                .prefix("max ammo: "),
                                        );
                                        ui.add(DragValue::new(&mut weapon.exp).prefix("exp: "));
                                    }
                                });
                            }
                        });
                    }
                });
            } else {
                #[cfg(not(target_arch = "wasm32"))]
                ui.label("Please load profile.dat");
            }

            ui.horizontal(|ui| {
                if let Some(raw) = &self.raw_profile {
                    if ui.button("Undo all").clicked() {
                        self.profile = Some(GameProfile::dump(raw));
                        self.weapon_num = self.count_weapon().unwrap();
                    }

                    if ui.button("Save").clicked() {
                        let mut raw = raw.clone();
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(path) = &self.path {
                            self.profile.unwrap().write(&mut raw);
                            let bytes: Vec<u8> = raw.into();
                            if let Err(e) = fs::write(path, bytes) {
                                use rfd::{MessageDialog, MessageLevel};
                                MessageDialog::new()
                                    .set_level(MessageLevel::Error)
                                    .set_description(&e.to_string())
                                    .set_title("Error occured while saving!")
                                    .show();
                            }
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            self.profile.unwrap().write(&mut raw);
                            let _bytes: Vec<u8> = raw.into();

                            // todo: export file with Blob API
                            todo!();
                        }
                    }
                }
            });

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
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
