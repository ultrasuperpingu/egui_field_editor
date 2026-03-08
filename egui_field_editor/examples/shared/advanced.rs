
use std::net::Ipv4Addr;

use egui::Color32;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Debug, Default)]
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
#[derive(EguiInspect, Debug, Default, PartialEq)]
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
	pub vector: Vec<TestData>,
	pub array: [TestData;4],
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
		Self { multiline: Default::default(), vector: Default::default(), array: Default::default(), u8: Default::default(), double: Default::default(), float: Default::default(), my_enum: Default::default(), char: Default::default(), ipv4: Ipv4Addr::UNSPECIFIED }
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
fn inspect_num(data: &mut i16, label: &str, tooltip:&str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) {
	egui_field_editor::add_number(data, label, tooltip, label_ratio, read_only, None, ui);
}
impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("advanced.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(self).with_title("Inpector").label_ratio(0.3));
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "Rust");
			});
		});
	}
}
