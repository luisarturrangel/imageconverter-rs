#![windows_subsystem = "windows"]
mod app;

fn load_icon(path: &str) -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    let min_size: [f32; 2] = [350.0, 170.0];
    // Icon handle
    let mut exe_dir = app::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    exe_dir.push("assets");
    exe_dir.push("icon.png");
    let icon_path_str = exe_dir.to_str().expect("Failed to convert path to string");
   
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(load_icon(icon_path_str))
            .with_inner_size(&min_size)
            .with_min_inner_size(&min_size),
        ..Default::default()
    };

    
    app::run_native(
        "imagecoverter-rs",
        options,
        Box::new(|_cc| Box::new(app::MyApp::default())),
    )
}
