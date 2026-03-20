
include!("shared/simple.rs");

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Field Editor Simple Example", options, 
		Box::new(|_cc|
			Ok(Box::new(MyApp {
				raw_string:"A raw string which is not editable, even is read_only=false",
				string: "A read only string".to_string(),
				..Default::default()
			}))
		)
	);
}