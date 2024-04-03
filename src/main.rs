use eframe::*;
use egui::{CentralPanel, Color32, Response, TextEdit, Window};
use std::sync::{Arc, Mutex};

struct MyApp {
    input_text: String,
    input_save: String,
    selected: Enum,
    error_visible: bool,
    block_input: bool,
    error: Option<ErrorType>,
    response_convert: Arc<Mutex<bool>>,
    loading: bool,
}

enum ErrorType {
    NoPathProvided,
    InvalidFileType,
}

impl ErrorType {
    fn error_menssage(error: &Option<ErrorType>) -> &str {
        let error = match error {
            Some(ErrorType::NoPathProvided) => "Error: No path provided",
            Some(ErrorType::InvalidFileType) => "Error: Invlid file type",
            _ => panic!("something"),
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
fn convert(path: &str, path_save: Option<&str>, file_type: &Enum) -> bool {
    let image_data = image::open(path).expect("Failed to open Image");
    let output_ext = match file_type {
        Enum::PNG => ".png",
        Enum::JPEG => ".jpeg",
        Enum::BMP => ".bmp",
        Enum::WEBP => ".WEBP",
        Enum::ICO => ".ICO",
    };

    let path_handle = match path_save {
        Some(path_save) if !path_save.is_empty() => format!("{}/output{}", path_save, output_ext),
        _ => format!("output{}", output_ext),
    };

    image_data
        .save_with_format(path_handle, Enum::from_index(file_type))
        .expect("Failed to save Image");

    true
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
            response_convert: Arc::new(Mutex::new(false)),
            loading: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let response_clone = self.response_convert.clone();
            if !self.block_input {
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
                        ui.add(TextEdit::singleline(&mut self.input_text));

                        if ui.button("open").clicked() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_dialog(None, None, nfd::DialogType::SingleFile)
                            {
                                self.input_text = path;
                            }
                        }
                    });

                    ui.add_space(10.0);
                    ui.label("save in");
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
                    egui::ComboBox::from_label("Save as")
                        .selected_text(format!("{:?}", &self.selected))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected, Enum::PNG, "PNG");
                            ui.selectable_value(&mut self.selected, Enum::JPEG, "JPEG");
                            ui.selectable_value(&mut self.selected, Enum::BMP, "BMP");
                            ui.selectable_value(&mut self.selected, Enum::WEBP, "WEBP");
                            ui.selectable_value(&mut self.selected, Enum::ICO, "ICO");
                        });
                    // text that appers in screen
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        const BUTTON_WIDTH: f32 = 20.0;
                        const BUTTON_HEIGHT: f32 = 10.0;
                        ui.add_space(ui.available_width() - &BUTTON_WIDTH * 4.0);
                        ui.style_mut().spacing.button_padding =
                            (BUTTON_WIDTH, BUTTON_HEIGHT).into();
                        if ui.button("Convert").clicked() {
                            // self.loading = true;
                            
                            if self.input_text.is_empty() {
                                self.error = Some(ErrorType::NoPathProvided);
                                self.error_visible = true;
                            } else {
                                let tmp = image::open(&self.input_text);
                                match tmp {
                                    Err(_) => {
                                        self.error = Some(ErrorType::InvalidFileType);
                                        self.error_visible = true;
                                    }
                                    Ok(_) => {
                                        // TODO: implement loading
                                        self.loading = true;
                                        let input_text = self.input_text.clone();
                                        let input_save = self.input_save.clone();
                                        let selected = self.selected.clone();
                                        let response_clone_thread = response_clone.clone();
                                        std::thread::spawn(move || {
                                            let response_convert = convert(&input_text, Some(&input_save), &selected);

                                            *response_clone_thread.lock().unwrap() = response_convert;
                                        });
                                        
                                    }
                                }
                            }
                        }
                    });
                });

                // EOF
            }
        });

        if self.loading {
            self.block_input = true;
            Window::new("loading")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .interactable(false)
                .collapsible(false).open(&mut true)
                .show(ctx, |ui| {
                    ui.set_min_size(egui::Vec2 {
                        x: (100.0),
                        y: (50.0),
                    });
                    ui.set_max_size(egui::Vec2 {
                        x: (150.0),
                        y: (100.0),
                    });
                    ui.add_space(15.0);
                    ui.vertical_centered(|ui| {
                        ui.spinner();
                    });
                    if *self.response_convert.lock().unwrap() {
                        self.loading = false;
                        self.block_input = false;
                    }
                });
        }

        if *self.response_convert.lock().unwrap() {
            self.block_input = true;
            Window::new("Success")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.style_mut().visuals.warn_fg_color = Color32::RED;
                    ui.set_min_size(egui::Vec2 {
                        x: (100.0),
                        y: (50.0),
                    });
                    ui.set_max_size(egui::Vec2 {
                        x: (150.0),
                        y: (100.0),
                    });
                    if self.input_save.is_empty() {
                        ui.label("saved in: current directory");
                    } else {
                        ui.label(format!("saved in: {}", &self.input_save));
                    }
                    ui.vertical_centered(|ui| {
                        ui.style_mut().spacing.button_padding = (25.0, 5.0).into();
                        ui.add_space(10.0);
                        if ui.button("OK").clicked() {
                            self.block_input = false;
                            *self.response_convert.lock().unwrap() = false;
                        }
                    })
                });
        }

        if self.error_visible {
            self.block_input = true;

            Window::new("Error")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.style_mut().visuals.warn_fg_color = Color32::RED;
                    ui.set_min_size(egui::Vec2 {
                        x: (100.0),
                        y: (50.0),
                    });
                    ui.set_max_size(egui::Vec2 {
                        x: (150.0),
                        y: (100.0),
                    });
                    let error = ErrorType::error_menssage(&self.error);
                    ui.label(error);
                    ui.vertical_centered(|ui| {
                        ui.style_mut().spacing.button_padding = (25.0, 5.0).into();
                        ui.add_space(10.0);
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
