// Don't really know why but rust-analyser still need a main if feature is not active...

#[cfg(feature = "datepicker")]
include!("shared/datepicker.rs");

#[cfg(feature = "datepicker")]
fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native(
		"EGui Field Editor Date Picker Example",
		options,
		Box::new(|_cc| Ok(Box::new(MyApp::default()))),
	);
}
#[cfg(not(feature = "datepicker"))]
compile_error!("You need to activate the `datepicker` feature to compile this example");
