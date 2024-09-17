use std::path::PathBuf;

use crate::{file_tree::FileTree, fstree::FsTree, prelude::*};
use walkdir::WalkDir;

pub fn draw(ctx: &egui::Context, tree: &FsTree) {
    egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
        // ctx.set_zoom_factor(0.5);
        // ctx.set_pixels_per_point(2.0);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.label("MuSMB");
            ui.menu_button("File", |ui| {
                ui.set_max_width(200.);

                ui.button("Connect...");
                if ui.button("Close").clicked() {
                    ui.close_menu();
                };
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(
            egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    FsTree::render_tree(ui, tree);
                });
            },
        );
    });
}
