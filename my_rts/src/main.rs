#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Theme};
use egui::{Color32, Id, Pos2, Stroke};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        vsync: true,
        multisampling: 8,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        run_and_return: true,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}
struct MyApp {
    name: String,
    age: u32,
    panel_shown: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            panel_shown: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!(
                r"C:\Users\User\Downloads\image-85.png"
            ));

            ui.painter().circle(
                Pos2::new(0.0, 0.0),
                20.0,
                Color32::from_rgb(255, 0, 0),
                Stroke::default(),
            );

            if ui.button("show panel").clicked() {
                self.panel_shown = !self.panel_shown;
            }

            if self.panel_shown {
                egui::SidePanel::new(egui::panel::Side::Left, Id::new("sziamia")).show(ctx, |ui| {
                    ui.heading("sziamia");
                });
            }
        });
    }
}
