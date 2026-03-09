
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;



#[derive(EguiInspect)]
struct MyApp{
	pub path: std::path::PathBuf,
	#[inspect(file(filter="*.jpg", filter="*.png"))]
	pub path2: std::path::PathBuf,
}
impl Default for MyApp {
	fn default() -> Self {
		Self { path: std::path::PathBuf::new(), path2: std::path::PathBuf::new() }
	}
}
impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("filepicker.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			if ui.add(EguiInspector::new(self).with_title("Inpector")).changed() {
				println!("Changed!!!")
			}
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "Rust");
			});
		});
	}
}
