use std::ops::{AddAssign, SubAssign};

use cavestory_save::{items::Inventory, strum::IntoEnumIterator};
use egui::{
    text::{LayoutJob, TextWrapping},
    TextFormat, Ui, Vec2,
};

const MAX_INVENTORY_NUM: usize = 31;

pub fn draw_window(ui: &mut Ui, inventory_num: &mut usize, inventory: &mut [Inventory]) {
    ui.horizontal(|ui| {
        let could_add = (*inventory_num == 0
            || inventory
                .get(*inventory_num - 1)
                .is_some_and(|&i| i != Inventory::None))
            // do not set the 8th weapon, you may go into issues.
            && *inventory_num < MAX_INVENTORY_NUM;
        let could_sub = *inventory_num > 0;

        if ui.button(" + ").clicked() && could_add {
            inventory_num.add_assign(1);
        }

        if ui.button(" - ").clicked() && could_sub {
            inventory_num.sub_assign(1);
            inventory[*inventory_num] = Default::default();
        }

        if ui.button(" x ").clicked() {
            *inventory_num = 0;
            inventory[..]
                .iter_mut()
                .for_each(|i| *i = Default::default());
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
                let pos = chunk_i * chunk_size + i;
                let truncated_warp = TextWrapping {
                    max_rows: 1,
                    break_anywhere: false,
                    ..Default::default()
                };
                let mut layout_job = LayoutJob::default();
                layout_job.append(&inventory.to_string(), 0., TextFormat::default());
                layout_job.wrap = truncated_warp;
                ui.scope(|ui| {
                    ui.spacing_mut().icon_spacing = 0.;
                    ui.spacing_mut().icon_width = 5.;
                    ui.spacing_mut().item_spacing = Vec2::from([3., 3.]);
                    ui.set_max_width(120.);
                    egui::ComboBox::new(format!("inventorytype-box-{pos}"), "")
                        .selected_text(layout_job)
                        .wrap()
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
