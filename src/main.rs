use eframe::*;
use egui::{CentralPanel, TextEdit};

struct MyApp {
    input_text: String,
    input_save: String,
    selected: Enum,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Enum {
    PNG,
    JPEG,
    BMP,
}

impl Enum {
    fn from_index(index: &Enum) -> image::ImageFormat {
        match index {
            Enum::PNG => image::ImageFormat::Png,
            Enum::JPEG => image::ImageFormat::Jpeg,
            Enum::BMP => image::ImageFormat::Bmp,
        }
    }
}

#[warn(unused_must_use)]
fn convert(path: &str, path_save: Option<&str>, file_type: &Enum) {
    let jpeg_image = image::open(path).expect("Failed to open Jpeg");
    let output_ext = match file_type {
        Enum::PNG => ".png",
        Enum::JPEG => ".jpeg",
        Enum::BMP => ".bmp",
    };

    let path_handle = match path_save {
        Some(path_save) if !path_save.is_empty() => format!("{}/output{}",path_save, output_ext),
        _ => format!("output{}", output_ext),
    };

    jpeg_image
        .save_with_format(path_handle, Enum::from_index(file_type))
        .expect("Failed to save Image");
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            input_save: String::new(),
            selected: Enum::PNG,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.input_text));

                if ui.button("open").clicked() {
                    if let Ok(nfd::Response::Okay(path)) =
                        nfd::open_dialog(None, None, nfd::DialogType::SingleFile)
                    {
                        self.input_text = path;
                    }
                }
            });
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", &self.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, Enum::PNG, "PNG");
                    ui.selectable_value(&mut self.selected, Enum::JPEG, "JPEG");
                    ui.selectable_value(&mut self.selected, Enum::BMP, "BMP");
                });

            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.input_save));
                let button_save = ui.button("chose");
                if button_save.clicked() {
                    if let Ok(nfd::Response::Okay(path)) =
                        nfd::open_dialog(None, None, nfd::DialogType::PickFolder)
                    {
                        self.input_save = path;
                    }
                }
            });
            // text that appers in screen

            let send_btn = ui.button("send");
            if send_btn.clicked() {
                convert(&self.input_text, Some(&self.input_save), &self.selected);
            }
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    run_native(
        "app_name",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
