// eframe 以图形模式运行
#![windows_subsystem = "windows"]

mod app;

fn main() {
    let option = eframe::NativeOptions {
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Todos",
        option,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(app::MyApp::new(cc)))),
    );
}
