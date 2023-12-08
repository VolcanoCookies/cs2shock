use std::{process, sync::Arc};

use eframe::icon_data::from_png_bytes;
use egui::{widgets::DragValue, Button, Id, ViewportBuilder};
use log::{debug, info};
use tokio::sync::RwLock;

use crate::{
    config::{Config, ShockMode},
    pishock,
};

pub async fn run(config: Arc<RwLock<Config>>) {
    let png_bytes = include_bytes!("../assets/icon.png");
    let viewport = ViewportBuilder::default()
        .with_inner_size([320.0, 360.0])
        .with_resizable(false)
        .with_icon(Arc::new(
            from_png_bytes(png_bytes).expect("Failed to load icon"),
        ));

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let changes = config.read().await.clone();
    let _ = eframe::run_native(
        "CS2 Shock",
        options,
        Box::new(|_cc| Box::new(MyApp { config, changes })),
    );
}

struct MyApp {
    config: Arc<RwLock<Config>>,
    changes: Config,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CS2 Shock");

            ui.horizontal(|ui: &mut egui::Ui| {
                let mut username_label_id = Id::NULL;
                ui.horizontal(|ui| {
                    ui.set_width(70.0);
                    username_label_id = ui.label("Username: ").id;
                });
                ui.text_edit_singleline(&mut self.changes.username)
                    .labelled_by(username_label_id);
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                let mut sharecode_label_id = Id::NULL;
                ui.horizontal(|ui| {
                    ui.set_width(70.0);
                    sharecode_label_id = ui.label("Share code: ").id;
                });
                ui.text_edit_singleline(&mut self.changes.code)
                    .labelled_by(sharecode_label_id);
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                let mut apikey_label_id = Id::NULL;
                ui.horizontal(|ui| {
                    ui.set_width(70.0);
                    apikey_label_id = ui.label("API key: ").id;
                });
                ui.text_edit_singleline(&mut self.changes.apikey)
                    .labelled_by(apikey_label_id);
            });

            ui.vertical_centered_justified(|ui| {
                ui.separator();
                let button = Button::new("Test beep");
                if ui.add(button).clicked() {
                    info!(target: "GUI", "Sending test beep");
                    let c = self.config.clone();
                    tokio::spawn(async move {
                        pishock::beep(c, 1).await;
                    });
                }
            });

            ui.vertical_centered(|ui| {
                ui.separator();
                ui.label("Shock Mode: ");
            });
            ui.vertical_centered_justified(|ui| {
                ui.selectable_value(&mut self.changes.shock_mode, ShockMode::Random, "Random");
                ui.selectable_value(
                    &mut self.changes.shock_mode,
                    ShockMode::LastHitPercentage,
                    "Last Hit Percentage",
                );
            });
            ui.vertical_centered(|ui| ui.separator());

            ui.horizontal(|ui| {
                let indensity_label = ui.label("Intensity: ");

                ui.add(
                    DragValue::new(&mut self.changes.min_intensity)
                        .speed(1)
                        .clamp_range(0..=self.changes.max_intensity)
                        .prefix("Min "),
                )
                .labelled_by(indensity_label.id);
                ui.add(
                    DragValue::new(&mut self.changes.max_intensity)
                        .speed(1)
                        .clamp_range(self.changes.min_intensity..=100)
                        .prefix("Max "),
                )
                .labelled_by(indensity_label.id);
            });
            ui.horizontal(|ui| {
                let duration_label = ui.label("Duration: ");
                ui.add(
                    DragValue::new(&mut self.changes.min_duration)
                        .speed(1)
                        .clamp_range(0..=self.changes.max_duration)
                        .prefix("Min "),
                )
                .labelled_by(duration_label.id);
                ui.add(
                    DragValue::new(&mut self.changes.max_duration)
                        .speed(1)
                        .clamp_range(self.changes.min_duration..=15)
                        .prefix("Max "),
                )
                .labelled_by(duration_label.id);
            });

            ui.add(egui::Checkbox::new(
                &mut self.changes.beep_on_match_start,
                "Beep on match start",
            ));
            ui.add(egui::Checkbox::new(
                &mut self.changes.beep_on_round_start,
                "Beep on round start",
            ));

            ui.vertical_centered(|ui| {
                ui.separator();
            });

            ui.vertical_centered_justified(|ui| {
                if let Ok(config) = self.config.try_read() {
                    let changed = config.to_owned() != self.changes;

                    if ui.add_enabled(changed, Button::new("Reset")).clicked() {
                        debug!(target: "GUI", "Resetting");
                        self.changes = config.to_owned();
                    }

                    if ui.add_enabled(changed, Button::new("Save")).clicked() {
                        debug!(target: "GUI", "Saving");
                        drop(config);
                        if let Ok(mut owned_config) = self.config.clone().try_write() {
                            *owned_config = self.changes.clone();
                            owned_config.write_to_file("config.json");
                        }
                    }
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                info!(target: "GUI", "Closing");
                process::exit(0);
            }
        });
    }
}
