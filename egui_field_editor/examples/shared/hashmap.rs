use std::collections::HashMap;
use egui::Color32;
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;

// Simple data type for values
#[derive(EguiInspect, Debug, Default, Clone)]
pub struct SimpleValue {
	#[inspect(name="Count")]
	pub count: u64,
	#[inspect(name="Active")]
	pub active: bool,
}

// More complex data type for values
#[derive(EguiInspect, Debug, Default, Clone, PartialEq)]
pub struct ComplexValue {
	#[inspect(name="ID")]
	pub id: u32,
	#[inspect(name="Name")]
	pub name: String,
	#[inspect(color)]
	pub color: Color32,
	#[inspect(name="Score", slider(min=0., max=100.))]
	pub score: f32,
}

// Main app showcasing different HashMap configurations
#[derive(EguiInspect)]
pub struct MyApp {
	#[inspect(name="Read-Only Key [Default]", tooltip="Cannot add, remove, or edit keys")]
	pub read_only_map: HashMap<String, u64>,

	#[inspect(name="Editable Values", hashmap(allow_add_delete=true, editable_keys=false))]
	pub editable_values: HashMap<String, SimpleValue>,

	#[inspect(name="Fully Editable", hashmap(allow_add_delete=true, editable_keys=true))]
	pub fully_editable: HashMap<String, String>,

	#[inspect(name="Complex Values [Read-Only Key]", tooltip="Complex data type inspection")]
	pub complex_readonly: HashMap<String, ComplexValue>,

	#[inspect(name="Complex Values [Editable Key]", hashmap(allow_add_delete=true, editable_keys=true))]
	pub complex_editable: HashMap<String, ComplexValue>,

	#[inspect(name="Custom Values Edition", hashmap(custom_fn="custom_hashmap_editor"))]
	pub custom_edit: HashMap<String, Option<ComplexValue>>,

	#[inspect(transparent=true)]
	pub transparent: HashMap<String, Option<ComplexValue>>,
}

impl Default for MyApp {
	fn default() -> Self {
		let mut read_only_map = HashMap::new();
		read_only_map.insert("key1".to_string(), 100u64);
		read_only_map.insert("key2".to_string(), 200u64);
		read_only_map.insert("key3".to_string(), 300u64);

		let mut editable_values = HashMap::new();
		editable_values.insert(
			"item_a".to_string(),
			SimpleValue {
				count: 42,
				active: true,
			},
		);
		editable_values.insert(
			"item_b".to_string(),
			SimpleValue {
				count: 17,
				active: false,
			},
		);

		let mut fully_editable = HashMap::new();
		fully_editable.insert("greeting".to_string(), "Hello".to_string());
		fully_editable.insert("farewell".to_string(), "Goodbye".to_string());

		let mut complex_readonly = HashMap::new();
		complex_readonly.insert(
			"alice".to_string(),
			ComplexValue {
				id: 1,
				name: "Alice".to_string(),
				color: Color32::from_rgb(255, 0, 0),
				score: 95.5,
			},
		);
		complex_readonly.insert(
			"bob".to_string(),
			ComplexValue {
				id: 2,
				name: "Bob".to_string(),
				color: Color32::from_rgb(0, 255, 0),
				score: 87.0,
			},
		);

		let mut complex_editable = HashMap::new();
		complex_editable.insert(
			"charlie".to_string(),
			ComplexValue {
				id: 3,
				name: "Charlie".to_string(),
				color: Color32::from_rgb(0, 0, 255),
				score: 75.5,
			},
		);
		let mut custom_edit = HashMap::new();
		custom_edit.insert(
			"charlie".to_string(),
			Some(ComplexValue {
				id: 3,
				name: "Charlie".to_string(),
				color: Color32::from_rgb(0, 0, 255),
				score: 75.5,
			})
		);
		custom_edit.insert(
			"alfred".to_string(),
			Some(ComplexValue {
				id: 8,
				name: "Alfred".to_string(),
				color: Color32::from_rgb(140, 25, 137),
				score: 15.2,
			})
		);
		custom_edit.insert("nobody".to_string(), None);
		let mut transparent = HashMap::new();
		transparent.insert(
			"charlie".to_string(),
			Some(ComplexValue {
				id: 3,
				name: "Charlie".to_string(),
				color: Color32::from_rgb(0, 0, 255),
				score: 75.5,
			})
		);
		transparent.insert(
			"alfred".to_string(),
			Some(ComplexValue {
				id: 8,
				name: "Alfred".to_string(),
				color: Color32::from_rgb(140, 25, 137),
				score: 15.2,
			})
		);
		Self {
			read_only_map,
			editable_values,
			fully_editable,
			complex_readonly,
			complex_editable,
			custom_edit,
			transparent
		}
	}
}

fn custom_hashmap_editor(
	item: &mut Option<ComplexValue>,
	parent_id: egui::Id,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	ui.vertical(|ui| {
		ui.label("This is a custom editor for the HashMap field! Hidding the enum...");
		if let Some(v) = item {
			v.inspect_with_custom_id(parent_id, label, tooltip, label_ratio, read_only, ui)
		} else {
			ui.response()
		}
	}).inner
}

impl eframe::App for MyApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		let code = include_str!("hashmap.rs");
		egui::Panel::right("right_panel")
			.resizable(true)
			.default_size(400.0)
			.show_inside(ui, |ui| {
				if ui.add(
					EguiInspector::new(self)
						.with_title("HashMap Showcase")
						.label_ratio(0.25),
				)
				.changed()
				{
					println!("Data changed!!")
				}
			});
		egui::CentralPanel::default().show_inside(ui, |ui| {
			egui::ScrollArea::vertical()
				.id_salt("code_scrolling")
				.show(ui, |ui| {
					use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
					code_view_ui(ui, &CodeTheme::default(), code, "Rust");
				});
		});
	}
}
