use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Default)]
struct MyApp {
	#[inspect(read_only)]
	string: String,
	#[inspect(multiline)]
	code: String,
	#[inspect(range(min = 12.0, max = 53.0))]
	unsigned32: u32,
	#[inspect(hidden)]
	#[allow(dead_code)]
	skipped: bool,
	#[inspect(tooltip = "A boolean")]
	boolean: bool,
	raw_string: &'static str,
	#[inspect(slider(min = "-43.0", max = 125.0))]
	float64: f64,
	#[inspect(name = "A proper field name")]
	ugly_internal_field_name: u16,
}

impl eframe::App for MyApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show_inside(ui, |ui| {
			if ui.add(EguiInspector::new(self)).changed() {
				println!("Changed!!!")
			}
		});
	}
}
