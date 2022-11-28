#[cfg(not(target_arch = "wasm32"))]
use std::{fs, path::PathBuf};

use cavestory_save::{GameProfile, Profile, ProfileError};

use cavestory_save::items::*;
use strum::IntoEnumIterator;

use egui::{DragValue, Slider};

#[derive(Default)]
pub struct MainApp {
    #[cfg(target_arch = "wasm32")]
    input: String,
    #[cfg(not(target_arch = "wasm32"))]
    path: Option<PathBuf>,
    profile: Option<(Profile, GameProfile)>,
    weapon_num: usize,
    equip_checked: [bool; 9],
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
                use rfd::{MessageDialog, MessageLevel};
                MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Load Error")
                    .set_description(&e.to_string())
                    .show();
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
                            let data: Vec<u8> = fs::read(&path).unwrap();
                            if self.verify_and_init(data).is_ok() {
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
                            if self.verify_and_init(bytes.into()).is_ok() {
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

            if let Some((
                _,
                GameProfile {
                    position,
                    map,
                    music,
                    health,
                    max_health,
                    weapon,
                    inventory: _,
                    teleporter: _,
                    equipment,
                },
            )) = &mut self.profile
            {
                egui::Window::new("Basic").show(ctx, |ui| {
                    ui.add(DragValue::new(health).prefix("heal: "));
                    ui.add(DragValue::new(max_health).prefix("max heal: "));

                    ui.label("BGM");
                    egui::ComboBox::new("background_music", "")
                        .selected_text(music.to_string())
                        .width(200.)
                        .show_ui(ui, |ui| {
                            for bg_music in Song::iter() {
                                ui.selectable_value(music, bg_music, bg_music.to_string());
                            }
                        });

                    ui.label("Map");
                    egui::ComboBox::new("map", "")
                        .selected_text(map.to_string())
                        .width(200.)
                        .show_ui(ui, |ui| {
                            for map_option in Map::iter() {
                                ui.selectable_value(map, map_option, map_option.to_string());
                            }
                        });

                    ui.label("Position");
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut position.x).prefix("x: "));
                        ui.add(DragValue::new(&mut position.y).prefix("y: "));
                    });
                });

                egui::Window::new("Equipments").show(ctx, |ui| {
                    for (i, equip) in Equipment::iter().enumerate() {
                        ui.checkbox(&mut self.equip_checked[i], equip.to_string());
                        equipment.switch(equip, self.equip_checked[i]);
                    }
                });

                egui::Window::new("Weapons").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // do not set the 8th weapon, you may go into issue.
                        if ui.button(" + ").clicked() && self.weapon_num < 7 {
                            self.weapon_num += 1
                        }
                        if ui.button(" - ").clicked() && self.weapon_num > 0 {
                            self.weapon_num -= 1;
                            weapon[self.weapon_num] = Weapon::default();
                        }
                    });

                    ui.separator();

                    for (chunk_i, chunk) in weapon[..self.weapon_num].chunks_mut(3).enumerate() {
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
                if let Some(profile) = &mut self.profile {
                    if ui.button("Undo all").clicked() {
                        profile.1 = GameProfile::dump(&profile.0);
                        let _x = profile;
                        self.weapon_num = self.count_weapon().unwrap();
                        self.equip_checked = self.detect_equip().unwrap();
                    }
                }

                if let Some(profile) = &self.profile {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Save").clicked() {
                        let mut modified_profile = profile.0.clone();
                        if let Some(path) = &self.path {
                            profile.1.write(&mut modified_profile);
                            let bytes: Vec<u8> = modified_profile.into();
                            if let Err(e) = fs::write(path, bytes) {
                                use rfd::{MessageDialog, MessageLevel};
                                MessageDialog::new()
                                    .set_level(MessageLevel::Error)
                                    .set_description(&e.to_string())
                                    .set_title("Error occured while saving!")
                                    .show();
                            }
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        let mut modified_profile = profile.0.clone();
                        profile.1.write(&mut modified_profile);
                        ui.hyperlink_to(
                            "Save",
                            format!(
                                "data:application/octet-stream;name=profile.dat;base64,{}",
                                base64::encode(Into::<Vec<u8>>::into(modified_profile))
                            ),
                        );
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
