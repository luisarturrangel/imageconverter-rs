use super::types::{ErrorType, FormatType};
pub use eframe::*;
pub use egui::{Color32, TextEdit, Window};
use std::{
    env,
    sync::{Arc, Mutex},
    path::PathBuf,
};

pub struct MyApp {
    input_text: String,
    input_save: String,
    selected: FormatType,
    error_visible: bool,
    block_input: bool,
    error: Option<ErrorType>,
    response_convert: Arc<Mutex<bool>>,
    loading: bool,
}

impl MyApp {
    fn draw_file_input(&mut self, ui: &mut egui::Ui) {
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
    }

    fn draw_save_input(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.label("save in");
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.input_save));
            let button_save = ui.button("choose");
            if button_save.clicked() {
                if let Ok(nfd::Response::Okay(path)) =
                    nfd::open_dialog(None, None, nfd::DialogType::PickFolder)
                {
                    self.input_save = path;
                }
            }
        });
    }

    fn draw_save_as_dropdown(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Save as")
            .selected_text(format!("{:?}", &self.selected))
            .show_ui(ui, |ui| {
                for format in FormatType::all() {
                    ui.selectable_value(&mut self.selected, *format, format.as_str());
                }
            });
    }

    fn draw_convert_button(&mut self, ui: &mut egui::Ui) -> bool {
        let mut response = false;
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            const BUTTON_WIDTH: f32 = 20.0;
            const BUTTON_HEIGHT: f32 = 10.0;
            ui.add_space(ui.available_width() - BUTTON_WIDTH * 4.0);
            ui.style_mut().spacing.button_padding = (BUTTON_WIDTH, BUTTON_HEIGHT).into();
            response = ui.button("Convert").clicked();
        });
        response
    }

    fn draw_image(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui_extras::install_image_loaders(ctx);
        let source = format!("file://{}", &self.input_text.clone());
        ui.add(
            egui::Image::new(source)
            .max_size(egui::Vec2 { 
                x: 190.0,
                y: 100.0,
            })
            .rounding(10.0)
            .show_loading_spinner(true)
        );
    }

    fn draw_loading_window(&mut self, ctx: &egui::Context) {
        Window::new("loading")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .fixed_rect(egui::Rect {
                min: egui::Pos2 { x: 0.0, y: 0.0 },
                max: egui::Pos2 { x: 0.0, y: 0.0 },
            })
            .interactable(false)
            .collapsible(false)
            .open(&mut true)
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

    fn draw_success_window(&mut self, ctx: &egui::Context) {
        Window::new("Success")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .fixed_rect(egui::Rect {
                min: egui::Pos2 { x: 0.0, y: 0.0 },
                max: egui::Pos2 { x: 0.0, y: 0.0 },
            })
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
                    let empty_path = env::current_dir().expect("Failed to find directory");
                    let empty_path = empty_path
                        .to_str()
                        .expect("Failed to convert PathBuf to string");
                    ui.label(format!("saved in: {}", empty_path));
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

    fn draw_error_window(&mut self, ctx: &egui::Context) {
        let pos = (egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0));
        Window::new("Error")
            .anchor(pos.0, pos.1)
            .fixed_rect(egui::Rect {
                min: egui::Pos2 { x: 0.0, y: 0.0 },
                max: egui::Pos2 { x: 0.0, y: 0.0 },
            })
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

    fn convert_image(&mut self) {
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
                    self.loading = true;
                    let input_text = self.input_text.clone();
                    let input_save = self.input_save.clone();
                    let selected = self.selected.clone();
                    let response_clone_thread = self.response_convert.clone();
                    std::thread::spawn(move || {
                        let response_convert = convert(&input_text, Some(&input_save), &selected);

                        *response_clone_thread.lock().unwrap() = response_convert;
                    });
                }
            }
        }
    }

    fn update_ui(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_size = ui.ctx().screen_rect();
            let content_width = 350.0; // Adjust based on your content
            let content_height = 170.0; // Adjust based on your content
            let padding_x = (window_size.width() - content_width) / 50.0;
            let padding_y = (window_size.height() - content_height) / 50.0;
            egui::containers::Frame::group(&egui::Style::default()).show(ui, |ui| {
                ui.style_mut().spacing.item_spacing.x = padding_x;
                ui.style_mut().spacing.item_spacing.y = padding_y;
                if !self.input_text.is_empty() {
                    self.draw_image(ui, ctx);
                } else {
                    ui.add_space(100.0);
                }
                self.draw_file_input(ui);
                self.draw_save_input(ui);
                self.draw_save_as_dropdown(ui);
                if self.draw_convert_button(ui) {
                    self.convert_image();
                }
            });

            if self.loading {
                self.draw_loading_window(ctx);
            }

            if *self.response_convert.lock().unwrap() {
                self.draw_success_window(ctx);
            }

            if self.error_visible {
                self.draw_error_window(ctx);
            }
        });
    }
}

#[warn(unused_must_use)]
fn convert(path: &str, path_save: Option<&str>, file_type: &FormatType) -> bool {
    let image_data = image::open(path).expect("Failed to open Image");
    let output_ext = FormatType::output_ext(file_type);
    
    let path_handle = match path_save {
        Some(path_save) if !path_save.is_empty() => {
            let path_buf = PathBuf::from(path);
            let file_stem = path_buf.file_stem().expect("Invalid file name").to_str().expect("Failed to convert file name to str");
            format!("{}/{}{}", path_save, file_stem, output_ext)
        },
        _ => {
        let path_buf = PathBuf::from(path);
        let file_stem = path_buf.file_stem()
                .expect("Invalid file name")
                .to_str()
                .expect("Failed to convert file name to str");
        format!("{}{}", file_stem, output_ext)},
    };

    image_data
        .save_with_format(path_handle, FormatType::from_index(file_type))
        .expect("Failed to save Image");

    true
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            input_save: String::new(),
            selected: FormatType::Png,
            error_visible: false,
            block_input: false,
            error: None,
            response_convert: Arc::new(Mutex::new(false)),
            loading: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.update_ui(ctx, frame);
    }
}
