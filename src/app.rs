use std::{fs, path::PathBuf};

use cavestory_save::{GameProfile, Profile};

use cavestory_save::items::*;
use strum::IntoEnumIterator;

use rfd::FileDialog;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    profile_path: Option<PathBuf>,

    #[serde(skip)]
    valid_profile: bool,

    #[serde(skip)]
    profile: Option<GameProfile>,

    #[serde(skip)]
    raw_profile: Option<Profile>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            profile_path: None,
            valid_profile: true,
            profile: None,
            raw_profile: None,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
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
                                self.raw_profile = Some(data);
                                self.valid_profile = true;
                                self.profile_path = Some(path);
                            } else {
                                self.valid_profile = false;
                                self.raw_profile = None;
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

            if !self.valid_profile {
                ui.label("Invalid Profile!");
            } else {
                if let Some(raw_profile) = &self.raw_profile {
                    let profile = self.profile.get_or_insert(GameProfile::dump(raw_profile));

                    ui.label("Heal");
                    ui.add(egui::Slider::new(&mut profile.health, -1..=50));

                    ui.label("Weapons");
                    for (i, weapon) in profile
                        .weapon
                        .iter_mut()
                        .enumerate()
                        .take_while(|(_, w)| w.classification != WeaponType::None)
                    {
                        egui::ComboBox::new(format!("weapontype-box-{i}"), format!("slot {i}"))
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
                    }
                }
            }

            ui.horizontal(|ui| {
                if self.raw_profile.is_some() {
                    if ui.button("Undo all").clicked() {
                        self.profile = None;
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
