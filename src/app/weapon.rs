use cavestory_save::{
    items::{Weapon, WeaponType},
    strum::IntoEnumIterator,
};
use egui::{DragValue, Slider, Ui};

pub fn draw_window(ui: &mut Ui, weapon_num: &mut usize, weapon: &mut [Weapon]) {
    ui.horizontal(|ui| {
        // do not set the 8th weapon, you may go into issue.
        if ui.button(" + ").clicked() && *weapon_num < 7 {
            *weapon_num += 1
        }
        if ui.button(" - ").clicked() && *weapon_num > 0 {
            *weapon_num -= 1;
            weapon[*weapon_num] = Weapon::default();
        }
    });

    ui.separator();

    for (chunk_i, chunk) in weapon[..*weapon_num].chunks_mut(3).enumerate() {
        ui.horizontal(|ui| {
            for (i, weapon) in chunk.iter_mut().enumerate() {
                ui.vertical(|ui| {
                    egui::ComboBox::new(format!("weapontype-box-{}", chunk_i * 3 + i), "")
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
                        ui.horizontal(|ui| {
                            ui.label("level");
                            ui.add(Slider::new(&mut weapon.level, 0..=3));
                        });
                        ui.horizontal(|ui| {
                            ui.label("ammo");
                            ui.add(DragValue::new(&mut weapon.ammo));
                        });
                        ui.horizontal(|ui| {
                            ui.label("max ammo");
                            ui.add(DragValue::new(&mut weapon.max_ammo));
                        });
                        ui.horizontal(|ui| {
                            ui.label("exp");
                            ui.add(DragValue::new(&mut weapon.exp));
                        });
                    }
                });
            }
        });
    }
}
