
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use nalgebra_glm::*;


use egui_field_editor::{EguiInspect, EguiInspector};

use eframe::egui;

#[derive(EguiInspect, Default, PartialEq)]
pub enum TestEnum {
	#[default]
	None,
	VectorsTuple(
		Vec2,
		Vec3,
		Vec4,
		TVec2<f64>,
		TVec3<f64>,
		TVec4<f64>,
		U8Vec2,
		U8Vec3,
		U8Vec4,
		I8Vec2,
		I8Vec3,
		I8Vec4,
		U16Vec2,
		U16Vec3,
		U16Vec4,
		I16Vec2,
		I16Vec3,
		I16Vec4,
		U32Vec2,
		U32Vec3,
		U32Vec4,
		I32Vec2,
		I32Vec3,
		I32Vec4,
		U64Vec2,
		U64Vec3,
		U64Vec4,
		I64Vec2,
		I64Vec3,
		I64Vec4,
		Quat,
		DQuat,
		Mat2x2,
		Mat2x3,
		Mat2x4,
		Mat3x2,
		Mat3x3,
		Mat3x4,
		Mat4x2,
		Mat4x3,
		Mat4x4,
	),
	ColorsTuple(
		#[inspect(color)]
		Vec3,
		#[inspect(color)]
		Vec4,
		#[inspect(color)]
		U8Vec3,
		#[inspect(color)]
		U8Vec4,
	),
	VectorsList(
		Vec<Vec3>,
	),
	VectorsNamed {
		vec2: Vec2,
		vec3: Vec3,
		vec4: Vec4,
		vec2f64: TVec2<f64>,
		vec3f64: TVec3<f64>,
		vec4f64: TVec4<f64>,
		vec2u8: U8Vec2,
		vec3u8: U8Vec3,
		vec4u8: U8Vec4,
		vec2i8: I8Vec2,
		vec3i8: I8Vec3,
		vec4i8: I8Vec4,
		vec2u16: U16Vec2,
		vec3u16: U16Vec3,
		vec4u16: U16Vec4,
		vec2i16: I16Vec2,
		vec3i16: I16Vec3,
		vec4i16: I16Vec4,
		vec2u32: U32Vec2,
		vec3u32: U32Vec3,
		vec4u32: U32Vec4,
		vec2i32: I32Vec2,
		vec3i32: I32Vec3,
		vec4i32: I32Vec4,
		vec2u64: U64Vec2,
		vec3u64: U64Vec3,
		vec4u64: U64Vec4,
		vec2i64: I64Vec2,
		vec3i64: I64Vec3,
		vec4i64: I64Vec4,
		quat:    Quat,
		dquat:   DQuat,
		mat2x2:  Mat2x2,
		mat2x3:  Mat2x3,
		mat2x4:  Mat2x4,
		mat3x2:  Mat3x2,
		mat3x3:  Mat3x3,
		mat3x4:  Mat3x4,
		mat4x2:  Mat4x2,
		mat4x3:  Mat4x3,
		mat4x4:  Mat4x4
	},
	ColorsNamed{
		#[inspect(color)]
		vec3: Vec3,
		#[inspect(color)]
		vec4: Vec4,
		#[inspect(color)]
		vec3u8: U8Vec3,
		#[inspect(color)]
		vec4u8: U8Vec4,
	},
}
#[derive(EguiInspect, Default)]
pub struct TestNamedStructColors {
	#[inspect(color)]
	pub vec3: Vec3,
	#[inspect(color)]
	pub vec4: Vec4,
	#[inspect(color)]
	pub vec3u8: U8Vec3,
	#[inspect(color)]
	pub vec4u8: U8Vec4,
}

#[derive(EguiInspect, Default)]
pub struct TestNamedStructVectors {
	pub vec2: Vec2,
	pub vec3: Vec3,
	pub vec4: Vec4,
	pub vec2f64: TVec2<f64>,
	pub vec3f64: TVec3<f64>,
	pub vec4f64: TVec4<f64>,
	pub vec2u8: U8Vec2,
	pub vec3u8: U8Vec3,
	pub vec4u8: U8Vec4,
	pub vec2i8: I8Vec2,
	pub vec3i8: I8Vec3,
	pub vec4i8: I8Vec4,
	pub vec2u16: U16Vec2,
	pub vec3u16: U16Vec3,
	pub vec4u16: U16Vec4,
	pub vec2i16: I16Vec2,
	pub vec3i16: I16Vec3,
	pub vec4i16: I16Vec4,
	pub vec2u32: U32Vec2,
	pub vec3u32: U32Vec3,
	pub vec4u32: U32Vec4,
	pub vec2i32: I32Vec2,
	pub vec3i32: I32Vec3,
	pub vec4i32: I32Vec4,
	pub vec2u64: U64Vec2,
	pub vec3u64: U64Vec3,
	pub vec4u64: U64Vec4,
	pub vec2i64: I64Vec2,
	pub vec3i64: I64Vec3,
	pub vec4i64: I64Vec4,
	pub quat:    Quat,
	pub dquat:   DQuat,
	pub mat2x2:  Mat2x2,
	pub mat2x3:  Mat2x3,
	pub mat2x4:  Mat2x4,
	pub mat3x2:  Mat3x2,
	pub mat3x3:  Mat3x3,
	pub mat3x4:  Mat3x4,
	pub mat4x2:  Mat4x2,
	pub mat4x3:  Mat4x3,
	pub mat4x4:  Mat4x4
}

#[derive(EguiInspect, Default)]
pub struct TestTupleStructColors(
	#[inspect(color)]
	pub Vec3,
	#[inspect(color)]
	pub Vec4,
	#[inspect(color)]
	pub U8Vec3,
	#[inspect(color)]
	pub U8Vec4,
);

#[derive(EguiInspect, Default)]
pub struct TestTupleStructVectors(
	pub Vec2,
	pub Vec3,
	pub Vec4,
	pub TVec2<f64>,
	pub TVec3<f64>,
	pub TVec4<f64>,
	pub U8Vec2,
	pub U8Vec3,
	pub U8Vec4,
	pub I8Vec2,
	pub I8Vec3,
	pub I8Vec4,
	pub U16Vec2,
	pub U16Vec3,
	pub U16Vec4,
	pub I16Vec2,
	pub I16Vec3,
	pub I16Vec4,
	pub U32Vec2,
	pub U32Vec3,
	pub U32Vec4,
	pub I32Vec2,
	pub I32Vec3,
	pub I32Vec4,
	pub U64Vec2,
	pub U64Vec3,
	pub U64Vec4,
	pub I64Vec2,
	pub I64Vec3,
	pub I64Vec4,
	pub Quat,
	pub DQuat,
	pub Mat2x2,
	pub Mat2x3,
	pub Mat2x4,
	pub Mat3x2,
	pub Mat3x3,
	pub Mat3x4,
	pub Mat4x2,
	pub Mat4x3,
	pub Mat4x4,
);



#[derive(Default, EguiInspect)]
struct MyApp {
	test_named_vectors:TestNamedStructVectors,
	test_named_colors:TestNamedStructColors,
	test_tuple_vectors:TestTupleStructVectors,
	test_tuple_colors:TestTupleStructColors,
	test_enum:TestEnum
}



impl eframe::App for MyApp {
	
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		let code = include_str!("nalgebra_glm.rs");
		egui::Panel::right("right_panel").show_inside(ui, |ui| {
			if ui.add(EguiInspector::new(self).with_title("Inpector")).changed() {
				println!("Changed!!!")
			}
		});
		egui::CentralPanel::default().show_inside(ui, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "Rust");
			});
		});
	}
}
