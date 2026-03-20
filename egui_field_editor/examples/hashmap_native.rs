include!("shared/hashmap.rs");

fn main() {
	let app = MyApp::default();
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native(
		"Egui Field Editor - HashMap Example",
		options,
		Box::new(|_cc| Ok(Box::new(app))),
	);
}
