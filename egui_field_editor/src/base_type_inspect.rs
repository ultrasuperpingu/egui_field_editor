use crate::EguiInspect;
use crate::collapsing_enum_variant_editor::CollapsingEnumVariantEditor;
use egui::text::CharIndex;
use egui::{Color32, Stroke, StrokeKind};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

macro_rules! impl_inspect_number {
	($($t:ty),+) => {
		$(
			impl crate::EguiInspect for $t {
				fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
					crate::add_number(self, label.into(), tooltip, label_ratio, read_only, None, ui)
				}
			}
		)*
	}
}

impl_inspect_number!(f32, f64);
impl_inspect_number!(i8, u8);
impl_inspect_number!(i16, u16);
impl_inspect_number!(i32, u32);
impl_inspect_number!(i64, u64);
//impl_inspect_number!(i128, u128);
impl_inspect_number!(isize, usize);

impl<T: EguiInspect> EguiInspect for &mut T {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		<T as EguiInspect>::inspect_with_custom_id(
			*self,
			parent_id,
			label,
			tooltip,
			label_ratio,
			read_only,
			ui,
		)
	}
}

impl<T: EguiInspect> EguiInspect for Box<T> {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		<T as EguiInspect>::inspect_with_custom_id(
			&mut *self,
			parent_id,
			label,
			tooltip,
			label_ratio,
			read_only,
			ui,
		)
	}
}

//Waiting for Specialization to be stable
/*impl<T: EguiInspect+std::fmt::Display> EguiInspect for Rc<RefCell<T>> {
	fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) {
		if let Ok(mut inner) = self.try_borrow_mut() {
			inner.inspect_with_custom_id(parent_id, label, tooltip, label_ratio, read_only, ui);
		} else if let Ok(inner) = self.try_borrow() {
			crate::add_string_multiline(*(inner.to_string()).into(), label, tooltip, label_ratio, true, 10, ui);
		} else {
			ui.label("🔒 Already borrowed");
		}
	}
}*/
impl<T: EguiInspect> EguiInspect for Rc<RefCell<T>> {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		if let Ok(mut inner) = self.try_borrow_mut() {
			inner.inspect_with_custom_id(parent_id, label, tooltip, label_ratio, read_only, ui)
		} else {
			ui.label("🔒 Already borrowed");
			ui.response()
		}
	}
}
impl<T: EguiInspect> EguiInspect for Arc<Mutex<T>> {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		match self.lock() {
			Ok(mut inner) => {
				inner.inspect_with_custom_id(parent_id, label, tooltip, label_ratio, read_only, ui)
			}
			Err(_) => {
				ui.label("❌ Failed to acquire lock");
				ui.response()
			}
		}
	}
}
impl<T: EguiInspect> EguiInspect for Arc<RwLock<T>> {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		match self.write() {
			Ok(mut inner) => {
				inner.inspect_with_custom_id(parent_id, label, tooltip, label_ratio, read_only, ui)
			}
			Err(_) => {
				ui.label("❌ Failed to acquire write lock");
				ui.response()
			}
		}
	}
}

impl crate::EguiInspect for &'static str {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		crate::add_string_singleline(self, label, tooltip, label_ratio, read_only, ui)
	}
}

impl crate::EguiInspect for String {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		crate::add_string_singleline(self, label, tooltip, label_ratio, read_only, ui)
	}
}

impl crate::EguiInspect for bool {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		crate::add_bool(self, label, tooltip, label_ratio, read_only, ui)
	}
}
struct CharString(String);
impl CharString {
	fn new(char: char) -> Self {
		let mut str = String::new();
		str.push(char);
		Self(str)
	}
	fn char(&self) -> char {
		self.0.chars().nth(0).unwrap() //safety: no method allow to get self.0.len() != 1
	}
}
impl egui::TextBuffer for CharString {
	fn is_mutable(&self) -> bool {
		true
	}
	fn as_str(&self) -> &str {
		self.0.as_str()
	}
	fn insert_text(&mut self, text: &str, _char_index: CharIndex) -> usize {
		if !text.is_empty() {
			let mut str = String::new();
			str.push(text.chars().nth(0).unwrap()); //safety: text is not empty so it has a first char
			self.0 = str;
		}
		0
	}
	fn delete_char_range(&mut self, _char_range: std::ops::Range<CharIndex>) {}

	fn type_id(&self) -> std::any::TypeId {
		std::any::TypeId::of::<Self>()
	}
}
impl crate::EguiInspect for char {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		let mut string = CharString::new(*self);
		let resp =
			crate::add_string_singleline(&mut string, label, tooltip, label_ratio, read_only, ui);
		if resp.changed() {
			*self = string.char();
		}
		resp
	}
}

/// Convenient struct to store a dragable item
struct EnumeratedItem<T> {
	item: T,
	index: usize,
	salt_id: egui::Id,
}

impl<T: crate::EguiInspect> egui_dnd::DragDropItem for EnumeratedItem<&mut T> {
	fn id(&self) -> egui::Id {
		egui::Id::new(self.salt_id.with(self.index))
	}
}
/// Shared function to inspect a collection with DnD support
fn inspect_collection<T: crate::EguiInspect>(
	items: &mut [T],
	parent_id: egui::Id,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	let id = if parent_id == egui::Id::NULL {
		ui.next_auto_id()
	} else {
		parent_id.with(label)
	};
	let parent_id_for_children = if parent_id == egui::Id::NULL {
		egui::Id::NULL
	} else {
		id
	};

	let mut changed = false;

	let collapsing_resp = egui::CollapsingHeader::new(format!("{label} [{}]", items.len()))
		.id_salt(id.with("collapse"))
		.show(ui, |ui| {
			let dnd_resp = egui_dnd::dnd(ui, id.with("dnd"))
				.with_animation_time(0.0)
				.show(
					items
						.iter_mut()
						.enumerate()
						.map(|(i, item)| EnumeratedItem {
							item,
							index: i,
							salt_id: id,
						}),
					|ui, item, handle, state| {
						ui.horizontal(|ui| {
							handle.ui(ui, |ui| {
								ui.label(if state.dragged { "≡" } else { "☰" });
							});

							let index = item.index;
							let res = item.item.inspect_with_custom_id(
								parent_id_for_children,
								&format!("Item {index}"),
								tooltip,
								label_ratio,
								read_only,
								ui,
							);

							if res.changed() {
								changed = true;
							}
						});
					},
				);

			if dnd_resp.is_drag_finished() {
				dnd_resp.update_vec(items);
				changed = true;
			}

			dnd_resp
		});

	let mut final_res = ui.response();
	if let Some(body_res) = collapsing_resp.body_response {
		final_res = final_res.union(body_res);
	}
	if changed {
		final_res.mark_changed();
	}

	final_res
}

impl<T: crate::EguiInspect, const N: usize> crate::EguiInspect for [T; N] {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		inspect_collection(self, _parent_id, label, tooltip, label_ratio, read_only, ui)
	}
}

impl<T: crate::EguiInspect + Default> crate::EguiInspect for Vec<T> {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		let mut res =
			inspect_collection(self, _parent_id, label, tooltip, label_ratio, read_only, ui);
		let mut changed = false;
		ui.add_enabled_ui(!read_only, |ui| {
			ui.horizontal_top(|ui| {
				ui.add_space(ui.available_width() - 50.);
				if ui
					.add(egui::Button::new("+").min_size(egui::Vec2::new(20., 20.)))
					.clicked()
				{
					self.push(T::default());
					changed = true;
				}
				#[allow(clippy::collapsible_if)]
				if ui
					.add(egui::Button::new("-").min_size(egui::Vec2::new(20., 20.)))
					.clicked()
				{
					if self.pop().is_some() {
						changed = true;
					}
				}
			});
		});
		if changed {
			res.mark_changed();
		}
		res
	}
}
impl<T: crate::EguiInspect> crate::EguiInspect for &mut [T] {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		inspect_collection(self, parent_id, label, tooltip, label_ratio, read_only, ui)
	}
}
impl<T: crate::EguiInspect + Default + Clone> crate::EguiInspect for HashMap<String, T> {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		crate::add_hashmap(
			self,
			parent_id,
			label,
			tooltip,
			label_ratio,
			read_only,
			false,
			false,
			ui,
		)
	}
}
impl crate::EguiInspect for Color32 {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		crate::add_color(self, label, tooltip, label_ratio, read_only, ui)
	}
}

impl crate::EguiInspect for Stroke {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		let mut add_content = |ui: &mut egui::Ui| {
			let resp = crate::add_color(
				&mut self.color,
				"Color",
				tooltip,
				label_ratio,
				read_only,
				ui,
			);
			let resp2 = crate::add_number(
				&mut self.width,
				"Width",
				tooltip,
				label_ratio,
				read_only,
				Some((0.0, 100.0)),
				ui,
			);
			resp.union(resp2)
		};
		if label.is_empty() {
			add_content(ui)
		} else {
			ui.collapsing(label, add_content)
				.body_response
				.unwrap_or(ui.response())
		}
	}
}

impl crate::EguiInspect for StrokeKind {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		let mut current_index = match self {
			StrokeKind::Inside => 0,
			StrokeKind::Middle => 1,
			StrokeKind::Outside => 2,
		};
		let resp = crate::add_combobox(
			&mut current_index,
			label,
			tooltip,
			label_ratio,
			read_only,
			&[
				"Inside".to_owned(),
				"Middle".to_owned(),
				"Outside".to_owned(),
			],
			ui,
		);
		if resp.changed() {
			match current_index {
				0 => *self = StrokeKind::Inside,
				1 => *self = StrokeKind::Middle,
				2 => *self = StrokeKind::Outside,
				_ => unreachable!(),
			}
		}
		resp
	}
}

impl crate::EguiInspect for std::path::PathBuf {
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		crate::add_path(self, label, tooltip, label_ratio, read_only, vec![], ui)
	}
}

impl<T: EguiInspect> crate::EguiInspect for Option<T>
where
	T: Default + PartialEq,
{
	fn inspect_with_custom_id(
		&mut self,
		_parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		let label_ratio = label_ratio.clamp(0.1, 0.9);
		let id = if _parent_id == egui::Id::NULL {
			ui.next_auto_id()
		} else {
			_parent_id.with(label)
		};
		let parent_id = if _parent_id == egui::Id::NULL {
			egui::Id::NULL
		} else {
			id
		};

		/*let mut index = if self.is_none() { 0 } else { 1 };
		let choices: [String; 2] = ["None".into(), "Some".into()];
		let main_res = CollapsingEnumVariantEditor::new(
			label,
			tooltip,
			label_ratio,
		)
		.default_open(self.is_some())
		.enabled(!read_only)
		.show(
			ui,
			self,
			&choices[index].clone(),
			|ui: &mut Ui, _edited_obj| {
				let mut changed = false;
				for (i, choice) in choices.iter().enumerate() {
					if ui.selectable_label(index == i, choice).clicked() {
						index = i;
						changed = true;
					}
				}
				let mut resp = ui.response();
				if changed {
					println!("{} {}",changed, index);
					resp.mark_changed();
				}
				resp
			},
			|ui, edited_obj| match edited_obj {
				None => ui.response(),
				Some(field0) => {
					field0.inspect_with_custom_id(parent_id, "", "", label_ratio, read_only, ui)
				}
			},
			true,
		);
		if let Some(resp) = main_res.combo_response {
			if resp.changed() {
				match index {
					0 => *self = None,
					1 => *self = Some(T::default()),
					_ => unreachable!(),
				}
			}
			if let Some(body_resp) = main_res.body_returned {
				resp.union(body_resp)
			} else {
				resp
			}
		} else {
			if let Some(body_resp) = main_res.body_returned {
				body_resp
			} else {
				ui.response()
			}
		}*/
		let mut new_value = self.is_some();

		let main_res = CollapsingEnumVariantEditor::new(label, tooltip, label_ratio)
			.default_open(self.is_some())
			.enabled(!read_only)
			.show(
				ui,
				self,
				if new_value { "Some" } else { "None" },
				|ui: &mut egui::Ui, _| {
					let mut changed = false;

					if ui.selectable_label(!new_value, "None").clicked() {
						new_value = false;
						changed = true;
					}

					if ui.selectable_label(new_value, "Some").clicked() {
						new_value = true;
						changed = true;
					}

					let mut resp = ui.response();
					if changed {
						resp.mark_changed();
					}
					resp
				},
				|ui, edited_obj| match edited_obj {
					None => ui.response(),
					Some(inner) => {
						inner.inspect_with_custom_id(parent_id, "", "", label_ratio, read_only, ui)
					}
				},
				true,
			);
		let mut response = if let Some(r) = main_res.body_returned {
			r
		} else {
			ui.response()
		};
		if let Some(resp) = main_res.combo_response
			&& resp.changed()
		{
			match (self.is_some(), new_value) {
				(false, true) => *self = Some(T::default()),
				(true, false) => *self = None,
				_ => {}
			}
			response.mark_changed();
			response = response.with_new_rect(resp.rect);
		}
		response
	}
}

#[cfg(feature = "nalgebra_glm")]
mod nalgebra_ui {
	use crate::Color32Wrapper;
	use crate::EguiInspect;
	use egui::Color32;
	use nalgebra_glm::*;

	macro_rules! impl_only_numbers_struct_inspect {
		($Type:ident, [$($field:ident),+]) => {
			impl EguiInspect for $Type {
				fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
					crate::add_custom_ui(label, tooltip, label_ratio, read_only, ui, |ui, _field_size| {
						ui.group(|ui| {
							ui.horizontal(|ui| {
								let mut combined_res: Option<egui::Response> = None;
								$(
									ui.label(stringify!($field));
									let res = ui.add(egui::DragValue::new(&mut self.$field).speed(0.1));
									if let Some(ref mut total) = combined_res {
										*total = total.union(res);
									} else {
										combined_res = Some(res);
									}
								)+
								combined_res.expect("Macro expanded with no fields")
							}).inner
						}).inner
					})
				}
			}
		};
	}
	macro_rules! impl_mat_inspect {
		($Type:ident, [$( [$($field:ident),+] ),+]) => {
			impl EguiInspect for $Type {
				fn inspect_with_custom_id(
					&mut self,
					_parent_id: egui::Id,
					label: &str,
					tooltip: &str,
					label_ratio: f32,
					read_only: bool,
					ui: &mut egui::Ui,
				) -> egui::Response {
					crate::add_custom_ui(label, tooltip, label_ratio, read_only, ui, |ui, _field_size| {
						ui.vertical(|ui| {
							ui.group(|ui| {
								let mut mat_res: Option<egui::Response> = None;
								$(
									let row_res = ui.horizontal(|ui| {
										let mut line_res: Option<egui::Response> = None;
										$(
											ui.label(stringify!($field));
											let res = ui.add(egui::DragValue::new(&mut self.$field).speed(0.1));
											if let Some(ref mut total) = line_res {
												*total = total.union(res);
											} else {
												line_res = Some(res);
											}
										)+
										line_res.expect("Empty row in matrix macro")
									}).inner;
									if let Some(ref mut total) = mat_res {
										*total = total.union(row_res);
									} else {
										mat_res = Some(row_res);
									}
								)+
								mat_res.expect("Empty matrix in macro")
							}).inner
						}).inner
					})
				}
			}
		};
	}

	impl_only_numbers_struct_inspect!(Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(DVec2, [x, y]);
	impl_only_numbers_struct_inspect!(DVec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(DVec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U8Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U8Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U8Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I8Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I8Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I8Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U16Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U16Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U16Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I16Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I16Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I16Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U32Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U32Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U32Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I32Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I32Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I32Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U64Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U64Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U64Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I64Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I64Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I64Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(Quat, [i, j, k, w]);
	impl_only_numbers_struct_inspect!(DQuat, [i, j, k, w]);
	impl_mat_inspect!(Mat2x2, [[m11, m12], [m21, m22]]);
	impl_mat_inspect!(Mat2x3, [[m11, m12, m13], [m21, m22, m23]]);
	impl_mat_inspect!(Mat2x4, [[m11, m12, m13, m14], [m21, m22, m23, m24]]);
	impl_mat_inspect!(Mat3x2, [[m11, m12], [m21, m22], [m31, m32]]);
	impl_mat_inspect!(Mat3x3, [[m11, m12, m13], [m21, m22, m23], [m31, m32, m33]]);
	impl_mat_inspect!(
		Mat3x4,
		[
			[m11, m12, m13, m14],
			[m21, m22, m23, m24],
			[m31, m32, m33, m34]
		]
	);
	impl_mat_inspect!(Mat4x2, [[m11, m12], [m21, m22], [m31, m32], [m41, m42]]);
	impl_mat_inspect!(
		Mat4x3,
		[
			[m11, m12, m13],
			[m21, m22, m23],
			[m31, m32, m33],
			[m41, m42, m43]
		]
	);
	impl_mat_inspect!(
		Mat4x4,
		[
			[m11, m12, m13, m14],
			[m21, m22, m23, m24],
			[m31, m32, m33, m34],
			[m41, m42, m43, m44]
		]
	);

	impl From<Color32Wrapper> for Vec3 {
		fn from(value: Color32Wrapper) -> Self {
			Vec3::new(
				value.0.r() as f32 / 255.,
				value.0.g() as f32 / 255.,
				value.0.b() as f32 / 255.,
			)
		}
	}
	impl From<Vec3> for Color32Wrapper {
		fn from(value: Vec3) -> Self {
			Self(Color32::from_rgb(
				(value.x * 255.) as u8,
				(value.y * 255.) as u8,
				(value.z * 255.) as u8,
			))
		}
	}

	impl From<Color32Wrapper> for Vec4 {
		fn from(value: Color32Wrapper) -> Self {
			Vec4::new(
				value.0.r() as f32 / 255.,
				value.0.g() as f32 / 255.,
				value.0.b() as f32 / 255.,
				value.0.a() as f32 / 255.,
			)
		}
	}
	impl From<Vec4> for Color32Wrapper {
		fn from(value: Vec4) -> Self {
			Self(Color32::from_rgba_premultiplied(
				(value.x * 255.) as u8,
				(value.y * 255.) as u8,
				(value.z * 255.) as u8,
				(value.w * 255.) as u8,
			))
		}
	}

	impl From<Color32Wrapper> for U8Vec3 {
		fn from(value: Color32Wrapper) -> Self {
			U8Vec3::new(value.0.r(), value.0.g(), value.0.b())
		}
	}
	impl From<U8Vec3> for Color32Wrapper {
		fn from(value: U8Vec3) -> Self {
			Self(Color32::from_rgb(value.x, value.y, value.z))
		}
	}

	impl From<Color32Wrapper> for U8Vec4 {
		fn from(value: Color32Wrapper) -> Self {
			U8Vec4::new(value.0.r(), value.0.g(), value.0.b(), value.0.a())
		}
	}
	impl From<U8Vec4> for Color32Wrapper {
		fn from(value: U8Vec4) -> Self {
			Self(Color32::from_rgba_premultiplied(
				value.x, value.y, value.z, value.w,
			))
		}
	}
}
#[cfg(feature = "datepicker")]
mod datepicker {
	#[cfg(feature = "chrono")]
	use crate::DateWrapper;
	use crate::EguiInspect;
	#[cfg(feature = "chrono")]
	use chrono::Datelike;
	use egui_extras::DatePickerButton;
	use std::hash::{Hash, Hasher};
	impl EguiInspect for jiff::civil::Date {
		fn inspect_with_custom_id(
			&mut self,
			parent_id: egui::Id,
			label: &str,
			tooltip: &str,
			label_ratio: f32,
			read_only: bool,
			ui: &mut egui::Ui,
		) -> egui::Response {
			let id = if parent_id == egui::Id::NULL {
				egui::Id::NULL
			} else {
				parent_id.with(label)
			};
			let widget = DatePickerButton::new(self);
			if id != egui::Id::NULL {
				// Ugly hack because DatePickerButton::id_salt() needs a &str
				let mut hasher = std::hash::DefaultHasher::new();
				id.hash(&mut hasher);
				crate::add_widget(
					label,
					widget.id_salt(format!("{}", hasher.finish()).as_str()),
					tooltip,
					label_ratio,
					read_only,
					ui,
				)
			} else {
				crate::add_widget(label, widget, tooltip, label_ratio, read_only, ui)
			}
		}
	}
	#[cfg(feature = "chrono")]
	impl EguiInspect for chrono::NaiveDate {
		fn inspect_with_custom_id(
			&mut self,
			parent_id: egui::Id,
			label: &str,
			tooltip: &str,
			label_ratio: f32,
			read_only: bool,
			ui: &mut egui::Ui,
		) -> egui::Response {
			let id = if parent_id == egui::Id::NULL {
				egui::Id::NULL
			} else {
				parent_id.with(label)
			};
			let mut jiff_date =
				jiff::civil::Date::new(self.year() as i16, self.month() as i8, self.day() as i8)
					.unwrap(); //TODO: fix this
			let widget = DatePickerButton::new(&mut jiff_date);
			let res = if id != egui::Id::NULL {
				// Ugly hack because DatePickerButton::id_salt() needs a &str
				let mut hasher = std::hash::DefaultHasher::new();
				id.hash(&mut hasher);
				crate::add_widget(
					label,
					widget.id_salt(format!("{}", hasher.finish()).as_str()),
					tooltip,
					label_ratio,
					read_only,
					ui,
				)
			} else {
				crate::add_widget(label, widget, tooltip, label_ratio, read_only, ui)
			};
			if res.changed()
				&& let Some(d) = chrono::NaiveDate::from_ymd_opt(
					jiff_date.year() as i32,
					jiff_date.month() as u32,
					jiff_date.day() as u32,
				) {
				*self = d;
			}
			res
		}
	}
	#[cfg(feature = "time")]
	impl EguiInspect for time::Date {
		fn inspect_with_custom_id(
			&mut self,
			parent_id: egui::Id,
			label: &str,
			tooltip: &str,
			label_ratio: f32,
			read_only: bool,
			ui: &mut egui::Ui,
		) -> egui::Response {
			let id = if parent_id == egui::Id::NULL {
				egui::Id::NULL
			} else {
				parent_id.with(label)
			};
			let mut jiff_date =
				jiff::civil::Date::new(self.year() as i16, self.month() as i8, self.day() as i8)
					.unwrap(); //TODO: fix this
			let widget = DatePickerButton::new(&mut jiff_date);
			let res = if id != egui::Id::NULL {
				// Ugly hack because DatePickerButton::id_salt() needs a &str
				let mut hasher = std::hash::DefaultHasher::new();
				id.hash(&mut hasher);
				crate::add_widget(
					label,
					widget.id_salt(format!("{}", hasher.finish()).as_str()),
					tooltip,
					label_ratio,
					read_only,
					ui,
				)
			} else {
				crate::add_widget(label, widget, tooltip, label_ratio, read_only, ui)
			};
			if res.changed() {
				*self = DateWrapper(jiff_date).into();
			}
			res
		}
	}
	#[cfg(feature = "chrono")]
	impl From<DateWrapper> for chrono::NaiveDate {
		fn from(value: DateWrapper) -> Self {
			//TODO: unwrap...
			chrono::NaiveDate::from_ymd_opt(
				value.0.year() as i32,
				value.0.month() as u32,
				value.0.day() as u32,
			)
			.unwrap()
		}
	}
	#[cfg(feature = "chrono")]
	impl From<chrono::NaiveDate> for DateWrapper {
		fn from(value: chrono::NaiveDate) -> Self {
			//TODO: unwrap...
			Self(
				jiff::civil::Date::new(value.year() as i16, value.month() as i8, value.day() as i8)
					.unwrap(),
			)
		}
	}
	#[cfg(feature = "time")]
	impl From<DateWrapper> for time::Date {
		fn from(value: DateWrapper) -> Self {
			//TODO: unwrap...
			time::Date::from_calendar_date(
				value.0.year() as i32,
				(value.0.month() as u8).try_into().unwrap(),
				value.0.day() as u8,
			)
			.unwrap()
		}
	}
	#[cfg(feature = "time")]
	impl From<time::Date> for DateWrapper {
		fn from(value: time::Date) -> Self {
			//TODO: unwrap...
			Self(
				jiff::civil::Date::new(value.year() as i16, value.month() as i8, value.day() as i8)
					.unwrap(),
			)
		}
	}
}

#[cfg(feature = "smallvec")]
mod smallvec {
	use crate::EguiInspect;
	use smallvec::SmallVec;

	impl<T, A> EguiInspect for SmallVec<A>
	where
		T: EguiInspect + Default,
		A: smallvec::Array<Item = T>,
	{
		fn inspect_with_custom_id(
			&mut self,
			parent_id: egui::Id,
			label: &str,
			tooltip: &str,
			label_ratio: f32,
			read_only: bool,
			ui: &mut egui::Ui,
		) -> egui::Response {
			let mut res = super::inspect_collection(
				self.as_mut_slice(),
				parent_id,
				label,
				tooltip,
				label_ratio,
				read_only,
				ui,
			);
			let mut changed = false;
			ui.add_enabled_ui(!read_only, |ui| {
				ui.horizontal_top(|ui| {
					ui.add_space(ui.available_width() - 50.);
					if ui
						.add(egui::Button::new("+").min_size(egui::Vec2::new(20., 20.)))
						.clicked()
					{
						self.push(T::default());
						changed = true;
					}
					if ui
						.add(egui::Button::new("-").min_size(egui::Vec2::new(20., 20.)))
						.clicked() && self.pop().is_some()
					{
						changed = true;
					}
				});
			});
			if changed {
				res.mark_changed();
			}
			res
		}
	}
}

#[cfg(feature = "arrayvec")]
mod arrayvec {
	use crate::EguiInspect;
	use arrayvec::ArrayVec;

	impl<T, const N: usize> EguiInspect for ArrayVec<T, N>
	where
		T: EguiInspect + Default,
	{
		fn inspect_with_custom_id(
			&mut self,
			parent_id: egui::Id,
			label: &str,
			tooltip: &str,
			label_ratio: f32,
			read_only: bool,
			ui: &mut egui::Ui,
		) -> egui::Response {
			let mut res = super::inspect_collection(
				self.as_mut_slice(),
				parent_id,
				label,
				tooltip,
				label_ratio,
				read_only,
				ui,
			);
			let mut changed = false;
			ui.add_enabled_ui(!read_only, |ui| {
				ui.horizontal_top(|ui| {
					ui.add_space(ui.available_width() - 50.);
					if ui
						.add_enabled(
							N > self.len(),
							egui::Button::new("+").min_size(egui::Vec2::new(20., 20.)),
						)
						.clicked()
					{
						self.push(T::default());
						changed = true;
					}
					if ui
						.add(egui::Button::new("-").min_size(egui::Vec2::new(20., 20.)))
						.clicked() && self.pop().is_some()
					{
						changed = true;
					}
				});
			});
			if changed {
				res.mark_changed();
			}
			res
		}
	}
}
