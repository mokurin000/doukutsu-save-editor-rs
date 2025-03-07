use cavestory_save::{
    items::{Map, Position, Song},
    strum::IntoEnumIterator,
};
use egui::{DragValue, Ui};

pub fn draw_window(
    ui: &mut Ui,
    health: &mut i16,
    max_health: &mut i16,
    music: &mut Song,
    map: &mut Map,
    position: &mut Position,
) {
    ui.horizontal(|ui| {
        ui.label("heal");
        ui.add(DragValue::new(health));
    });
    ui.horizontal(|ui| {
        ui.label("max heal ");
        ui.add(DragValue::new(max_health));
    });

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
        ui.label("x: ");
        ui.add(DragValue::new(&mut position.x));
        ui.label("y: ");
        ui.add(DragValue::new(&mut position.y));
    });
}
