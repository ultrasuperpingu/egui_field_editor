#[cfg(feature = "nalgebra_glm")]
include!("shared/nalgebra_glm.rs");

#[cfg(feature = "nalgebra_glm")]
fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native(
		"Egui Field Editor NAlgebra-glm Example",
		options,
		Box::new(|_cc| Ok(Box::new(MyApp::default()))),
	);
}
#[cfg(not(feature = "nalgebra_glm"))]
compile_error!("You need to activate the `nalgebra_glm` feature to compile this example");
