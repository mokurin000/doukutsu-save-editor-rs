use cavestory_save::{
    items::{Weapon, WeaponType},
    strum::IntoEnumIterator,
};
use egui::{DragValue, Slider, Ui};

const MAX_WEAPON_NUM: usize = 7;

pub fn draw_window(ui: &mut Ui, weapon_num: &mut usize, weapon: &mut [Weapon]) {
    ui.horizontal(|ui| {
        // do not set the 8th weapon, you may go into issue.
        if (*weapon_num == 0
            || weapon
                .get(*weapon_num - 1)
                .is_some_and(|w| w.classification != WeaponType::None))
            && *weapon_num < MAX_WEAPON_NUM
            && ui.button(" + ").clicked()
        {
            *weapon_num += 1
        }

        if *weapon_num > 0 && ui.button(" - ").clicked() {
            *weapon_num -= 1;
            weapon[*weapon_num] = Weapon::default();
        }
    });

    ui.separator();

    let chunk_size = 3;
    for (chunk_i, chunk) in weapon[..*weapon_num].chunks_mut(chunk_size).enumerate() {
        ui.horizontal(|ui| {
            for (i, weapon) in chunk.iter_mut().enumerate() {
                ui.vertical(|ui| {
                    let pos = chunk_i * chunk_size + i;
                    egui::ComboBox::new(format!("weapontype-box-{pos}"), "")
                        .width(150.)
                        .selected_text(weapon.classification.to_string())
                        .show_ui(ui, |ui| {
                            let mut iter = WeaponType::iter();
                            if pos + 1 < *weapon_num {
                                iter.next();
                            }
                            for model in iter {
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
