// Don't really know why but rust-analyser still need a main if feature is not active...

#[cfg(feature = "filepicker")]
include!("shared/filepicker.rs");

#[cfg(feature = "filepicker")]
fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native(
		"EGui Field Editor File Picker Example",
		options,
		Box::new(|_cc| Ok(Box::new(MyApp::default()))),
	);
}
#[cfg(not(feature = "filepicker"))]
compile_error!("You need to activate the `filepicker` feature to compile this example");
