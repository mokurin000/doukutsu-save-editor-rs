use cavestory_save::{items::Inventory, strum::IntoEnumIterator};
use egui::Ui;

const MAX_INVENTORY_NUM: usize = 31;

pub fn draw_window(ui: &mut Ui, inventory_num: &mut usize, inventory: &mut [Inventory]) {
    ui.horizontal(|ui| {
        // do not set the 8th weapon, you may go into issue.
        if (*inventory_num == 0
            || inventory
                .get(*inventory_num - 1)
                .is_some_and(|&i| i != Inventory::None))
            && *inventory_num < MAX_INVENTORY_NUM
            && ui.button(" + ").clicked()
        {
            *inventory_num += 1
        }

        if *inventory_num > 0 && ui.button(" - ").clicked() {
            *inventory_num -= 1;
            inventory[*inventory_num] = Default::default();
        }
    });

    ui.separator();

    let chunk_size = 6;
    for (chunk_i, chunk) in inventory[..*inventory_num]
        .chunks_mut(chunk_size)
        .enumerate()
    {
        ui.horizontal(|ui| {
            for (i, inventory) in chunk.iter_mut().enumerate() {
                ui.vertical(|ui| {
                    let pos = chunk_i * chunk_size + i;
                    egui::ComboBox::new(format!("weapontype-box-{pos}"), "")
                        .width(150.)
                        .selected_text(inventory.to_string())
                        .show_ui(ui, |ui| {
                            let mut iter = Inventory::iter();
                            if pos + 1 < *inventory_num {
                                iter.next();
                            }
                            for model in iter {
                                ui.selectable_value(inventory, model, model.to_string());
                            }
                        });
                });
            }
        });
    }
}
