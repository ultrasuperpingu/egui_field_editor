
use std::net::Ipv4Addr;
use std::collections::HashMap;

use egui::Color32;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Debug, Default, Clone)]
pub struct TestData(
	#[inspect(name="Name", tooltip="You can name tuple field")]
	String,
	#[inspect(name="A Float read only", read_only=true)]
	f32,
	#[inspect(name="A Float slider", slider(min=12.,max=155.))]
	f32,
	#[inspect(color, tooltip="not named => \"Field 3\"")]
	Color32,
	#[inspect(name="A Boxed string")]
	Box<String>,
	#[inspect(hidden)]
	#[allow(dead_code)]
	MyEnum
);
#[derive(EguiInspect, Debug, Default, PartialEq, Clone)]
pub enum MyEnum {
	#[default]
	UnitVariant,
	TupleVariant1(
		#[inspect(slider(min=1.,max=12.))]
		u8
	),
	TupleVariant2(
		#[inspect(name="Renamed Variant tuple field")]
		u8,
		#[inspect(custom_fn="inspect_num")]
		i16
	),
	NamedVariant{a:f32, b:i64},
	#[inspect(hidden)]
	IgnoredVariant,
	#[inspect(name="MyRenamedVariant")]
	RenamedVariant,
	#[inspect(read_only)]
	ReadOnlyVariant(u8, u8),
	SomeFieldReadOnlyVariantNamed {
		a:u8,
		#[inspect(read_only)]
		b:u8,
		#[inspect(hidden)]
		c:u8,
	},
	SomeFieldReadOnlyVariantTuple(
		u8,
		#[inspect(read_only)]
		u8,
		#[inspect(hidden)]
		u8,
	)
}

#[derive(EguiInspect)]
#[inspect(execute_btn(fn_name="println_hello", is_method=false), execute_btn(fn_name="set_double_field_to_pi", tooltip="3.141592653"))]
struct MyApp {
	#[inspect(multiline=8)]
	pub multiline:String,
	pub test_data: TestData,
	#[inspect(transparent=true)]
	pub transparent_test_data: TestData,
	pub vector: Vec<TestData>,
	pub array: [TestData;4],
	#[cfg(feature = "smallvec")]
	pub smallvec_array: smallvec::SmallVec<[TestData;4]>,
	#[cfg(feature = "arrayvec")]
	pub array_vec: arrayvec::ArrayVec<TestData,4>,
	pub hashmap: HashMap<String, u64>,
	pub u8: u8,
	#[inspect(range(min = 0., max = 12.0))]
	pub double: f64,
	#[inspect(slider(min = "-1000.", max = 12.0))]
	pub float: f32,
	pub my_enum:MyEnum,
	pub char:char,
	#[inspect(from_string)]
	pub ipv4: Ipv4Addr
}
impl Default for MyApp {
	fn default() -> Self {
		let mut hashmap = HashMap::new();
		hashmap.insert("value1".into(), 0);
		hashmap.insert("value2".into(), 12);
		Self {
			multiline: Default::default(),
			test_data: Default::default(),
			transparent_test_data: Default::default(),
			vector: Default::default(),
			array: Default::default(),
			#[cfg(feature = "smallvec")]
			smallvec_array: smallvec::SmallVec::new(),
			#[cfg(feature = "arrayvec")]
			array_vec: arrayvec::ArrayVec::new(),
			hashmap,
			u8: Default::default(),
			double: Default::default(),
			float: Default::default(),
			my_enum: Default::default(),
			char: Default::default(), 
			ipv4: Ipv4Addr::UNSPECIFIED
		}
	}
}
impl MyApp {
	fn set_double_field_to_pi(&mut self) {
		self.double = 3.1415;
	}
}
fn println_hello() {
	println!("Hello");
}
fn inspect_num(data: &mut i16, label: &str, tooltip:&str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
	egui_field_editor::add_number(data, label, tooltip, label_ratio, read_only, None, ui)
}
impl eframe::App for MyApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		let code = include_str!("advanced.rs");
		egui::Panel::right("right_panel").show_inside(ui, |ui| {
			if ui.add(EguiInspector::new(self).with_title("Inpector").label_ratio(0.3)).changed() {
				println!("Changed!!")
			}
		});
		egui::CentralPanel::default().show_inside(ui, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "Rust");
			});
		});
	}
}
