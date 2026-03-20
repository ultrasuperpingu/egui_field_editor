
#![no_main]
#![cfg(target_arch = "wasm32")]

use eframe::wasm_bindgen::prelude::*;

mod shared {
	pub mod hashmap_showcase;
}

use shared::hashmap_showcase::HashMapShowcaseApp;

#[wasm_bindgen]
pub async fn main_web(canvas_id: &str) {
	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		let _ = eframe::WebRunner::new()
			.start(
				canvas_id,
				web_options,
				Box::new(|_cc| Ok(Box::new(HashMapShowcaseApp::default()))),
			)
			.await;
	});
}
