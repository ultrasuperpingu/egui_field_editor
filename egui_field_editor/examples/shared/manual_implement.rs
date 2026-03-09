use std::net::Ipv4Addr;
use egui::Color32;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::EguiInspector;

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("manual_implement.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(self).with_title("Inpector"));
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "Rust");
			});
		});
	}
}
#[derive(better_default::Default)]
struct MyApp {
	a_bool:bool,
	an_int:i32,
	an_uint:u64,
	a_float:f32,
	#[default(Color32::from_rgb(127, 0, 200))]
	a_color:egui::Color32,
	#[default(String::from("A single line string"))]
	a_string:String,
	#[default(String::from("A\nmultiline\nline\nstring"))]
	a_second_string:String,
	an_index:usize,
	#[default(Ipv4Addr::UNSPECIFIED)]
	an_ipv4:Ipv4Addr
}
impl egui_field_editor::EguiInspect for MyApp {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, _tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
		let _parent_id_to_provide_to_children = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		let mut add_content=|ui:&mut egui::Ui| {
			let mut res: Option<egui::Response> = None;

			let mut combine = |r: egui::Response| {
				if let Some(ref mut total) = res {
					*total = total.union(r);
				} else {
					res = Some(r);
				}
			};
			combine(egui_field_editor::add_bool(&mut self.a_bool, "Bool", "Boolean Tooltip", label_ratio, read_only, ui));
			combine(egui_field_editor::add_number(&mut self.an_int, "Integer", "Integer Tooltip", label_ratio, read_only, None, ui));
			combine(egui_field_editor::add_number(&mut self.an_uint, "Unsigned Integer", "Unsigned Integer Tooltip with min/max", label_ratio, read_only, Some((12, 50000)), ui));
			combine(egui_field_editor::add_number_slider(&mut self.a_float, "Float", "Float Slider Tooltip", label_ratio, read_only, -12., 50., ui));
			combine(egui_field_editor::add_color(&mut self.a_color, "Color", "", label_ratio, read_only, ui));
			combine(egui_field_editor::add_string_singleline(&mut self.a_string, "String", "", label_ratio, read_only, ui));
			combine(egui_field_editor::add_string_multiline(&mut self.a_second_string, "Multiline String", "", label_ratio, read_only, 4, ui));
			combine(egui_field_editor::add_combobox(&mut self.an_index, "Combobox", "", label_ratio, read_only, &["Choice 1".to_string(),"Choice 2".to_string(),"Choice 3".to_string()], ui));
			combine(egui_field_editor::add_string_convertible(&mut self.an_ipv4, "IPv4", "", label_ratio, false, ui));
			res.unwrap_or_else(|| ui.response())
		};
		if !label.is_empty() {
			let header_resp = egui::CollapsingHeader::new(label).id_salt(id).show(ui, add_content);
			let mut final_res = header_resp.header_response;
			if let Some(body) = header_resp.body_response {
				final_res = final_res.union(body);
			}
			final_res
		} else {
			add_content(ui)
		}
	}
}
