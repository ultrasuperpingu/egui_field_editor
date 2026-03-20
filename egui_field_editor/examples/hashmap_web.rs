
#![no_main]
#![cfg(target_arch = "wasm32")]

use eframe::web_sys;
use eframe::wasm_bindgen::JsCast;

include!("shared/hashmap.rs");

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
	let app = MyApp::default();
	let web_options = eframe::WebOptions::default();
	let runner = eframe::WebRunner::new();

	let canvas = web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.get_element_by_id("canvas_id"))
		.and_then(|elem| elem.dyn_into::<web_sys::HtmlCanvasElement>().ok())
		.expect("Failed to get canvas element");

	wasm_bindgen_futures::spawn_local(async move {
		let _ = runner
			.start(canvas, web_options, Box::new(|_cc| Ok(Box::new(app))))
			.await;
	});

	Ok(())
}
