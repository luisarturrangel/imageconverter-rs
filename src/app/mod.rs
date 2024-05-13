#[warn(unused_imports)]
pub use eframe::*;
pub use egui::{CentralPanel, Color32, TextEdit, Window};
pub use std::{env, path::PathBuf, sync::{Arc, Mutex}};

pub mod app;
pub mod types;

pub use app::MyApp;
pub use types::{ErrorType, FormatType};