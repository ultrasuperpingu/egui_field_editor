
include!("shared/advanced.rs");

fn main() {
	let app = MyApp::default();
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Field Editor Advanced Example", options, Box::new(|_cc| Ok(Box::new(app))));
}

