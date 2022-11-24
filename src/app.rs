use std::mem::zeroed;
use std::{fs, path::PathBuf};

use cavestory_save::{GameProfile, Profile};

use cavestory_save::items::*;
use strum::IntoEnumIterator;

use rfd::FileDialog;

pub struct App {
    path: Option<PathBuf>,
    valid: bool,
    profile: Option<GameProfile>,
    raw_profile: Option<Profile>,
    weapon_num: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            path: None,
            valid: true,
            profile: None,
            raw_profile: None,
            weapon_num: 0,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    fn count_weapon(&self) -> Option<usize> {
        self.profile.and_then(|p| {
            Some(
                p.weapon.len()
                    - p.weapon
                        .iter()
                        .rev()
                        .take_while(|w| w.classification == WeaponType::None)
                        .count(),
            )
        })
    }

    fn dump_profile(&mut self) {
        if let Some(raw_profile) = &self.raw_profile {
            self.profile = Some(GameProfile::dump(&raw_profile));
            self.weapon_num = self.count_weapon().unwrap();
        }
    }
}

impl eframe::App for App {
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
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let path = FileDialog::default()
                            .add_filter("Profile", &["dat"])
                            .set_title("Pick your game profile")
                            .pick_file(); // this file is guranteed availiable to read
                        if let Some(path) = path {
                            let data: Profile = fs::read(&path).unwrap().into();
                            if data.verify() {
                                self.path = Some(path);
                                self.valid = true;

                                self.raw_profile = Some(data);
                                self.dump_profile();
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
            if false {} // todo: pick/save file on web

            ui.set_max_width(50.);

            if !self.valid {
                ui.label("Invalid Profile!");
            } else {
                if let Some(_) = &self.raw_profile {
                    if self.profile.is_none() {
                        self.dump_profile();
                    }

                    let profile = &mut self.profile.unwrap();
                    ui.label("Heal");
                    ui.add(egui::Slider::new(&mut profile.health, -1..=50));

                    ui.label("Weapons");
                    ui.horizontal(|ui| {
                        for (i, weapon) in profile.weapon[..self.weapon_num].iter_mut().enumerate()
                        {
                            ui.vertical(|ui| {
                                egui::ComboBox::new(
                                    format!("weapontype-box-{i}"),
                                    format!("slot {i}"),
                                )
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
                                    // attributes here
                                    ui.label("test");
                                }
                            });
                        }

                        // do not set the 8th weapon, you may go into issue.
                        if self.weapon_num < 7 && ui.button("add").clicked() {
                            self.weapon_num += 1
                        }
                        if self.weapon_num > 0 && ui.button("del").clicked() {
                            self.weapon_num -= 1;
                            profile.weapon[self.weapon_num] = Weapon::default();
                        }
                    });
                }
            }

            ui.horizontal(|ui| {
                if self.raw_profile.is_some() {
                    if ui.button("Undo all").clicked() {
                        self.dump_profile();
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

    fn on_close_event(&mut self) -> bool {
        true
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
