use crate::{fstree::FsTree, prelude::*};

pub fn draw(ctx: &egui::Context, tree: &FsTree) {
    egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.label("MuSMB");
            ui.menu_button("File", |ui| {
                ui.set_max_width(200.);

                let _ = ui.button("Connect...");
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
