use eframe::*;
use egui::{CentralPanel, TextEdit, Ui, Window};

struct MyApp {
    input_text: String,
    input_save: String,
    selected: Enum,
    error_visible: bool,
    error: Option<ErrorType>,
    block_input: bool,
}
enum ErrorType {
    NoPathProvided,
    InvalidFileType,
}

#[warn(unreachable_patterns)]
impl ErrorType {
    fn error_message(error: &ErrorType) -> &str {
        let error = match error {
            ErrorType::NoPathProvided => "Error: No path provided",
            ErrorType::InvalidFileType => "Error: Invalid file type",
            _ => panic!(),
        };
        error
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Enum {
    PNG,
    JPEG,
    BMP,
    WEBP,
    ICO,
}

impl Enum {
    fn from_index(index: &Enum) -> image::ImageFormat {
        match index {
            Enum::PNG => image::ImageFormat::Png,
            Enum::JPEG => image::ImageFormat::Jpeg,
            Enum::BMP => image::ImageFormat::Bmp,
            Enum::WEBP => image::ImageFormat::WebP,
            Enum::ICO => image::ImageFormat::Ico,
        }
    }
}

#[warn(unused_must_use)]
impl MyApp {
    fn convert(&mut self, path: &str, path_save: Option<&str>, file_type: &Enum) {
        let output_ext = match file_type {
            Enum::PNG => ".png",
            Enum::JPEG => ".jpeg",
            Enum::BMP => ".bmp",
            Enum::WEBP => ".WEBP",
            Enum::ICO => ".ICO",
        };
        let image_data = image::open(path);
        match image_data {
            Err(_) => {
                self.error = Some(ErrorType::InvalidFileType);
                self.error_visible = true;
            }
            Ok(image) => {
                let path_handle = match path_save {
                    Some(path_save) if !path_save.is_empty() => {
                        format!("{}/output{}", path_save, output_ext)
                    }
                    _ => format!("output{}", output_ext),
                };

                // Save the image with the specified output format
                image
                    .save_with_format(&path_handle, Enum::from_index(file_type))
                    .expect("Failed to save Image");
            }
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            input_save: String::new(),
            selected: Enum::PNG,
            error_visible: false,
            block_input: false,
            error: None,
        }
    }
}

fn update_logic(my_app: &mut MyApp, ctx: &egui::Context, ui: &mut Ui) -> egui::Response {
    let mut convert_btn = ui.button("Convert");
    if !my_app.block_input {
        // SOF
        let window_size = ui.ctx().screen_rect();
        let content_width = 350.0; // Adjust based on your content
        let content_height = 170.0; // Adjust based on your content
        let padding_x = (window_size.width() - content_width) / 50.0;
        let padding_y = (window_size.height() - content_height) / 50.0;
        egui::containers::Frame::group(&egui::Style::default()).show(ui, |ui| {
            ui.style_mut().spacing.item_spacing.x = padding_x;
            ui.style_mut().spacing.item_spacing.y = padding_y;
            ui.label("File");
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut my_app.input_text));

                if ui.button("open").clicked() {
                    if let Ok(nfd::Response::Okay(path)) =
                        nfd::open_dialog(None, None, nfd::DialogType::SingleFile)
                    {
                        my_app.input_text = path;
                    }
                }
            });

            ui.add_space(10.0);
            ui.label("save in");
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut my_app.input_save));
                let button_save = ui.button("chose");
                if button_save.clicked() {
                    if let Ok(nfd::Response::Okay(path)) =
                        nfd::open_dialog(None, None, nfd::DialogType::PickFolder)
                    {
                        my_app.input_save = path;
                    }
                }
            });
            egui::ComboBox::from_label("Save as")
                .selected_text(format!("{:?}", &my_app.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut my_app.selected, Enum::PNG, "PNG");
                    ui.selectable_value(&mut my_app.selected, Enum::JPEG, "JPEG");
                    ui.selectable_value(&mut my_app.selected, Enum::BMP, "BMP");
                    ui.selectable_value(&mut my_app.selected, Enum::WEBP, "WEBP");
                    ui.selectable_value(&mut my_app.selected, Enum::ICO, "ICO");
                });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                const BUTTON_WIDTH: f32 = 20.0;
                const BUTTON_HEIGHT: f32 = 10.0;
                ui.add_space(ui.available_width() - &BUTTON_WIDTH * 4.0);
                ui.style_mut().spacing.button_padding = (BUTTON_WIDTH, BUTTON_HEIGHT).into(); 
                convert_btn = ui.button("Convert");

            });

        });
    }
    // EOF
    convert_btn
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let convert_btn = update_logic(self, ctx, ui);
            if convert_btn.clicked() {
                if self.input_text.is_empty() {
                    self.error = Some(ErrorType::NoPathProvided);
                    self.error_visible = true;
                } else {
                    self.convert(&self.input_text, Some(&self.input_save), &self.selected);
                }
            }
        });

        // Error handler
        if self.error_visible {
            self.block_input = true;
            Window::new("Error")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.set_min_size(egui::Vec2 {
                        x: (80.0),
                        y: (40.0),
                    });
                    ui.set_max_size(egui::Vec2 {
                        x: (100.0),
                        y: (50.0),
                    });
                    let Some(error_tp) = self.error;
                    ui.label(ErrorType::error_message(&error_tp));
                    ui.vertical_centered(|ui| {
                        ui.style_mut().spacing.button_padding = (25.0, 5.0).into();
                        if ui.button("OK").clicked() {
                            self.error_visible = false;
                            self.block_input = false;
                        }
                    })
                });
        }
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    let min_size: [f32; 2] = [350.0, 170.0];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(&min_size)
            .with_min_inner_size(&min_size),
        ..Default::default()
    };
    run_native(
        "app_name",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
