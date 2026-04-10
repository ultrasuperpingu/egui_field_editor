#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::invalid_html_tags)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

//! This crate expose macros and traits to generate boilerplate code
//! for structs inspection and edition.
//!
//! Basic usage would be:
//! ```
//! use egui_field_editor::{EguiInspect, EguiInspector};
//! use eframe::egui;
//!
//! #[derive(EguiInspect, Default)]
//! struct MyApp {
//!     #[inspect(read_only)]
//!     string: String,
//!     #[inspect(multiline)]
//!     code: String,
//!     #[inspect(range(min = 12.0, max = 53.0))]
//!     unsigned32: u32,
//!     #[inspect(hidden)]
//!     #[allow(dead_code)]
//!     skipped: bool,
//!     #[inspect(tooltip = "A boolean")]
//!     boolean: bool,
//!     raw_string: &'static str,
//!     #[inspect(slider(min = "-43.0", max = 125.0))]
//!     float64: f32,
//!     #[inspect(name = "A proper field name")]
//!     ugly_internal_field_name: u16,
//! }
//!
//! impl eframe::App for MyApp {
//!     fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show_inside(ui, |ui| {
//!             ui.add(EguiInspector::new(self));
//!         });
//!     }
//! }
//!
//! fn main() {
//!     let options = eframe::NativeOptions::default();
//!     let _ = eframe::run_native("EGui Inspector Very Simple Example", options,
//!         Box::new(|_cc|
//!             Ok(Box::new(MyApp {
//!                 raw_string:"A raw string which is not editable, even is read_only=false",
//!                 string: "A read only string".to_string(),
//!                 ..Default::default()
//!             }))
//!         )
//!     );
//! }
//! ```
//!
//! You can add attributes to structures field.
//!
//! Currently supported attributes are defined in the struct AttributeArgs of egui_field_editor_derive
//!
//! Here is a list of supported attributes:
//!
//! - `name` *(String)*: Use custom label for the given field instead of the internal field name
//! - `hidden` *(bool)*: If true, doesn't generate code for the given field
//! - `read_only` *(bool)*: If true, the field is not editable (and color is grayed)
//! - `slider` *(min=f32, max=f32)*: If present, use a slider when inspecting numbers
//! - `range` *(min=f32, max=f32)*: Min/Max value for inspecting numbers
//! - `multiline` *(optional u8)*: If set, display the text on multiple lines. If affected to a u8, it defines the number of rows to display
//! - `tooltip` *(String)*: Tooltip to display when cursor is hover
//! - `color` *(bool)*: Display the field has a color (field type needs to implement [`From<Color32Wrapper>`]/[`Into<Color32Wrapper>`] - see [`Color32Wrapper`])
//! - `custom_fn` *(String)*: Use a custom function instead of calling [`EguiInspect::inspect_with_custom_id`]
//! - `from_string`: *(bool)*: Force edition from string conversion (needs type to implement [`FromStr`] and [`Display`])
//!
//!    Compatible with `multiline`.`
//! - `date` *(DatePickerParams)*: Parameters to customize the Date Picker widget:
//!   - ```combo_boxes```: *(optional ```bool```)*
//!     Show combo boxes in date picker popup. (Default: true).
//!   - ```arrows```: *(optional ```bool```)*
//!     Show arrows in date picker popup. (Default: true).
//!   - ```calendar```: *(optional ```bool```)*
//!     Show calendar in date picker popup. (Default: true).
//!   - ```calendar_week```: *(optional ```bool```)*
//!     Show calendar week in date picker popup. (Default: true).
//!   - ```show_icon```: *(optional ```bool```)*
//!     Show the calendar icon on the button. (Default: true).
//!   - ```format```: *```String```*
//!     Change the format shown on the button. (Default: ```"%Y-%m-%d"```).
//!
//!     See [`chrono::format::strftime`] for valid formats.
//!
//!   - ```highlight_weekends```: (optional ```bool```).
//!     Highlight weekend days. (Default: true)
//!   - ```start_end_years```: (min = ```String```|```i32```, max = ```String```|```i32```):
//!
//!     Set the start and end years for the date picker. (Default: today's year - 100 to today's year + 10)
//!
//!     This will limit the years you can choose from in the dropdown to the specified range.
//!
//!     For example, if you want to provide the range of years from 2000 to 2035, you can use: `start_end_years(min=2000, max=2035)`.
//!
//! # Feature Flags
//! This crate provides optional features to extend functionality with external libraries. You can enable them ²ively to reduce compile time and dependency footprint.
//! - `nalgebra_glm`: Enables support for inspecting nalgebra-glm types like Vec3, Vec4, etc.
//!
//!   This adds a dependency to [nalgebra-glm](https://docs.rs/nalgebra-glm/latest/nalgebra_glm/index.html).
//! - `datepicker`: Enables date picker UI using jiff and egui_extras.
//!
//!   This adds a dependency to [egui_extras](https://docs.rs/egui_extras/latest/egui_extras/index.html) datepicker feature and to [jiff](https://docs.rs/jiff/latest/jiff/).
//!   This implement inspector for the [jiff::civil::Date] type.
//!   You can optionally use features [chrono](https://docs.rs/chrono/latest/chrono/) and [time](https://docs.rs/time/latest/time/) to implement it for This implement inspector for the [time::Date] and [chrono::NaiveDate].
//! - `filepicker`: Enables file picker UI using rfd.
//!
//! - `smallvec`: Enables EguiInpect implementation for [smallvec::SmallVec].
//!
//! - `arrayvec`: Enables EguiInpect implementation for [arrayvec::ArrayVec].
//!
//! - `all`: A shortcut to activate all features.
//!
//!
//! ##  Default Features
//! No features are activated by default.
//! ```toml
//! default = []
//! ```

use egui::{Color32, Response, Ui, Widget};
#[cfg(feature = "nalgebra_glm")]
use nalgebra_glm::*;
use std::{
	fmt::Display,
	ops::{Deref, DerefMut},
	str::FromStr,
};
#[cfg(feature = "datepicker")]
use std::{
	hash::{Hash, Hasher},
	ops::RangeInclusive,
};

/// See also [EguiInspect]
pub use egui_field_editor_derive::*;

/// A wrapper widget that renders an object implementing [`EguiInspect`] inside an `egui` UI.
///
/// This struct provides a convenient way to embed an inspector view for any type that
/// implements the [`EguiInspect`] trait. It supports toggling read-only mode and integrates
/// seamlessly with `egui`'s layout system. It provide a separator in the header allowing to change the label size.
///
/// # Type Parameters
///
/// - `T`: The type to inspect, which must implement [`EguiInspect`].
///
/// # Examples
///
/// ```rust
/// use egui_field_editor::{EguiInspector, EguiInspect};
/// #[derive(EguiInspect, Default, PartialEq)]
/// enum MyConfig {
///     #[default]
///     None,
///     ByName(String),
///     ById(u32),
///     ByNetwork{hostname:String, port:u16},
/// }
/// let mut config = MyConfig::default();
/// let inspector = EguiInspector::new(&mut config);
/// //ui.add(inspector);
/// ```
///
/// # See Also
///
/// - [`EguiInspect`]
/// - [`egui::Widget`]
pub struct EguiInspector<'a, T: EguiInspect + ?Sized> {
	obj: &'a mut T,
	title: Option<String>,
	read_only: bool,
	id_salt: Option<egui::Id>,
	label_ratio: f32,
}
impl<'a, T: EguiInspect + ?Sized> EguiInspector<'a, T> {
	/// Creates a new inspector widget for the given object.
	///
	/// - `obj`: The object to inspect.
	pub fn new(obj: &'a mut T) -> Self {
		Self {
			obj,
			title: None,
			read_only: false,
			id_salt: None,
			label_ratio: 0.3,
		}
	}
	/// Creates a new read only inspector widget for the given object.
	///
	/// - `obj`: The object to inspect.
	pub fn new_read_only(obj: &'a mut T) -> Self {
		Self {
			obj,
			title: None,
			read_only: true,
			id_salt: None,
			label_ratio: 0.3,
		}
	}
	/// Set read-only mode.
	#[inline]
	pub fn read_only(mut self) -> Self {
		self.read_only = true;
		self
	}
	/// A source for the unique [`egui::Id`], e.g. `.id_salt("inspector")` or `.id_salt(loop_index)`.
	#[inline]
	pub fn id_salt(mut self, id_salt: impl std::hash::Hash) -> Self {
		self.id_salt = Some(egui::Id::new(id_salt));
		self
	}
	/// Set a title for the widget.
	#[inline]
	pub fn with_title(mut self, title: &str) -> Self {
		self.title = Some(title.to_owned());
		self
	}
	/// Set label ratio.
	#[inline]
	pub fn label_ratio(mut self, size: f32) -> Self {
		self.label_ratio = size;
		self
	}
}

impl<'a, T: EguiInspect + ?Sized> Widget for EguiInspector<'a, T> {
	fn ui(self, ui: &mut Ui) -> Response {
		ui.set_min_width(100.);
		let id = self.id_salt.unwrap_or(ui.next_auto_id());

		if let Some(title) = &self.title {
			ui.heading(title);
		}

		let mut label_ratio = ui
			.ctx()
			.data_mut(|data| data.get_persisted::<f32>(id).unwrap_or(0.3));

		let splitter_width = 4.0;
		let available_width = ui.available_width();
		let splitter_x = label_ratio * available_width;

		let splitter_rect = egui::Rect::from_min_size(
			ui.min_rect().min + egui::vec2(splitter_x, 0.0),
			egui::vec2(splitter_width, ui.available_height()),
		);

		let splitter_resp =
			ui.interact(splitter_rect, ui.id().with("splitter"), egui::Sense::drag());

		let response: Option<Response> = ui.ctx().read_response(id);
		let state = response.map(|r| r.widget_state()).unwrap_or_default();
		let stroke = ui.style().separator_style(state).stroke;
		ui.painter().vline(
			splitter_rect.center().x,
			splitter_rect.top()..=splitter_rect.top() + 20.0,
			stroke,
		);
		if splitter_resp.hovered() || splitter_resp.dragged() {
			ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
		}
		if splitter_resp.dragged() {
			let delta = ui.input(|i| i.pointer.delta().x);
			label_ratio = (label_ratio + delta / available_width).clamp(0.1, 0.8);
			ui.ctx()
				.data_mut(|data| data.insert_persisted(id, label_ratio));
		}

		egui::ScrollArea::vertical()
			.id_salt(id.with("scroll"))
			.show(ui, |ui| {
				ui.set_min_width(available_width);

				if let Some(salt) = self.id_salt {
					self.obj
						.inspect_with_custom_id(salt, "", "", label_ratio, self.read_only, ui)
				} else {
					self.obj.inspect("", "", label_ratio, self.read_only, ui)
				}
			})
			.inner
	}
}

#[cfg(feature = "nalgebra_glm")]
macro_rules! impl_only_numbers_struct_inspect {
	($method:ident, $Type:ident, [$($field:ident),+]) => {
		#[doc = concat!("Adds an editor for [`", stringify!($Type), "`] using `egui::DragValue` for each field.")]
		#[doc = " "]
		#[doc = "# Parameters"]
		#[doc = "- `data`: Mutable reference to the `$Type` instance."]
		#[doc = "- `label`: Label displayed above the group."]
		#[doc = "- `tooltip`: Optional tooltip shown when hovering over the label."]
		#[doc = "- `read_only`: If true, disables interaction."]
		#[doc = "- `ui`: The `egui::Ui` to render into."]
		#[doc = " "]
		#[doc = "# See Also"]
		#[doc = "- [`egui::DragValue`]"]
		pub fn $method(data: &mut $Type, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
			crate::add_custom_ui(label, tooltip, label_ratio, read_only, ui, |ui, _field_size| {
				ui.horizontal(|ui| {
					let mut combined_res: Option<egui::Response> = None;
					$(
						ui.label(stringify!($field));
						let res = ui.add(egui::DragValue::new(&mut data.$field).speed(0.1));
						if let Some(ref mut total) = combined_res {
							*total = total.union(res);
						} else {
							combined_res = Some(res);
						}
					)+
					combined_res.expect("Macro expanded with no fields")
				}).inner
			})
		}
	}
}

#[cfg(feature = "nalgebra_glm")]
macro_rules! impl_mat_inspect {
	($method:ident, $Type:ident, [$( [$($field:ident),+] ),+]) => {
		#[doc = concat!("Adds an editor for [`", stringify!($Type), "`] using `egui::DragValue` for each field.")]
		#[doc = " "]
		#[doc = "# Parameters"]
		#[doc = "- `data`: Mutable reference to the `$Type` instance."]
		#[doc = "- `label`: Label displayed above the group."]
		#[doc = "- `tooltip`: Optional tooltip shown when hovering over the label."]
		#[doc = "- `read_only`: If true, disables interaction."]
		#[doc = "- `ui`: The `egui::Ui` to render into."]
		#[doc = " "]
		#[doc = "# See Also"]
		#[doc = "- [`egui::DragValue`]"]
		pub fn $method(data: &mut $Type, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
				crate::add_custom_ui(label, tooltip, label_ratio, read_only, ui, |ui, _field_size| {
					ui.vertical(|ui| {
						ui.group(|ui| {
							let mut mat_res: Option<egui::Response> = None;
							$(
								let row_res = ui.horizontal(|ui| {
									let mut line_res: Option<egui::Response> = None;
									$(
										ui.label(stringify!($field));
										let res = ui.add(egui::DragValue::new(&mut data.$field).speed(0.1));
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
	}

/// A trait for rendering custom UI inspectors using `egui`.
///
/// It is designed to simplify the creation of property editors or debug panels.
///
/// # Overview
///
/// - Use [`Self::inspect`] or [`Self::inspect_with_custom_id`] to start rendering a UI block.
/// - All widgets support tooltips and read-only mode.
/// - Layout is responsive: labels and fields are proportionally sized.
///
/// # Example
///
/// ```rust
/// #[derive(Default)]
/// struct MyStruct {
///     a_bool:bool,
///     an_int:i32,
///     an_uint:u64,
///     a_float:f32,
///     a_color:egui::Color32,
///     a_string:String,
///     a_second_string:String,
/// }
/// impl egui_field_editor::EguiInspect for MyStruct {
///     fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
///         let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
///         let _parent_id_to_provide_to_children = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
///         let mut add_content=|ui:&mut egui::Ui| {
///             egui_field_editor::add_bool(&mut self.a_bool, "Bool", "Boolean Tooltip", label_ratio, read_only, ui);
///             egui_field_editor::add_number(&mut self.an_int, "Integer", "Integer Tooltip", label_ratio, read_only, None, ui);
///             egui_field_editor::add_number(&mut self.an_uint, "Unsigned Integer", "Unsigned Integer Tooltip with min/max", label_ratio, read_only, Some((12, 50000)), ui);
///             egui_field_editor::add_number_slider(&mut self.a_float, "Float", "Float Slider Tooltip", label_ratio, read_only, -12., 50., ui);
///             egui_field_editor::add_color(&mut self.a_color, "Color", "", label_ratio, read_only, ui);
///             egui_field_editor::add_string_singleline(&mut self.a_string, "String", "", label_ratio, read_only, ui);
///             egui_field_editor::add_string_multiline(&mut self.a_second_string, "Multiline String", "", label_ratio, read_only, 4, ui);
///         };
///         if !label.is_empty() {
///             egui::CollapsingHeader::new(label).id_salt(id).show(ui, add_content);
///         } else {
///             add_content(ui);
///         }
///         //TODO: handle response correctly (detect changes in add content and returns the corresponding response)
///         ui.response()
///     }
/// }
/// ```
pub trait EguiInspect {
	/// Renders the inspector UI for this object.
	///
	/// This is a convenience method that delegates to [`Self::inspect_with_custom_id`] using a [NULL Id](egui::Id::NULL).
	///
	/// - `label`: Label displayed above the inspector block.
	/// - `tooltip`: Tooltip shown when hovering over the label.
	/// - `read_only`: If `true`, disables all interactive widgets.
	/// - `ui`: The `egui::Ui` to render into.
	fn inspect(
		&mut self,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response {
		self.inspect_with_custom_id(egui::Id::NULL, label, tooltip, label_ratio, read_only, ui)
	}
	/// Renders the inspector UI with a custom parent ID.
	///
	/// This allows you to scope widget IDs under a specific parent, useful for avoiding collisions.
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		label_ratio: f32,
		read_only: bool,
		ui: &mut egui::Ui,
	) -> egui::Response;
}

/// Adds a labeled widget to the UI with layout and tooltip support.
///
/// If `read_only` is set to `true`, the slider will be disabled and the value cannot be changed.
/// A tooltip will be shown when the user hovers over the label.
///
/// - `label`: Label shown to the left of the widget.
/// - `widget`: The widget to render.
/// - `tooltip`: Tooltip shown when hovering over the label.
/// - `label_ratio`: The width percent for the label.
/// - `read_only`: If `true`, disables the widget.
/// - `ui`: The `egui::Ui` to render into.
///
/// # See Also
///
/// - [`egui::Widget`]
/// - [add_custom_ui]
pub fn add_widget<T: egui::Widget>(
	label: &str,
	widget: T,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	crate::add_custom_ui(
		label,
		tooltip,
		label_ratio,
		read_only,
		ui,
		|ui, field_width| {
			ui.spacing_mut().slider_width = field_width - 50.;
			ui.add_sized([field_width, 0.], widget)
		},
	)
}
/// Adds a custom field with layout and tooltip support.
///
/// If `read_only` is set to `true`, the slider will be disabled and the value cannot be changed.
/// A tooltip will be shown when the user hovers over the label.
///
/// - `label`: Label shown to the left of the field.
/// - `tooltip`: Tooltip shown when hovering over the label.
/// - `label_ratio`: The width percent for the label.
/// - `read_only`: If `true`, disables the field.
/// - `ui`: The `egui::Ui` to render into.
/// - `field_renderer`: A closure that renders the field, receiving the available field width.
pub fn add_custom_ui<F>(
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
	field_renderer: F,
) -> egui::Response
where
	F: FnOnce(&mut egui::Ui, f32) -> egui::Response,
{
	let label_ratio = label_ratio.clamp(0.05, 0.95);

	let available_width = ui.available_width();
	let label_width = available_width * label_ratio;
	let field_width = 100.0f32.max(available_width * (1.0 - label_ratio) - 10.0);

	let inner_res = ui.horizontal_top(|ui| {
		let inner_enabled_res = ui.add_enabled_ui(!read_only, |ui| {
			let (rect, _label_res) = ui.allocate_exact_size(
				egui::vec2(label_width, ui.spacing().interact_size.y),
				egui::Sense::hover(),
			);

			let mut child_ui = ui.new_child(
				egui::UiBuilder::new()
					.max_rect(rect)
					.layout(egui::Layout::left_to_right(egui::Align::Min)),
			);

			let mut label_res = child_ui.add(
				egui::Label::new(label)
					.truncate()
					.show_tooltip_when_elided(true)
					.halign(egui::Align::LEFT),
			);

			if !tooltip.is_empty() {
				if !read_only {
					label_res = label_res.on_hover_text(tooltip);
				} else {
					label_res = label_res.on_disabled_hover_text(tooltip);
				}
			}

			let widget_res = field_renderer(ui, field_width);
			label_res.union(widget_res)
		});
		inner_enabled_res.inner
	});

	inner_res.inner
}

/// Adds a numeric slider to the given `egui` UI.
///
/// This function creates a horizontal slider widget that allows the user to adjust a numeric value
/// within a specified range. It supports any type that implements [`egui::emath::Numeric`], such as
/// `f32`, `f64`, `i32`, etc.
///
/// If `read_only` is set to `true`, the slider will be disabled and the value cannot be changed.
/// A tooltip will be shown when the user hovers over the label.
///
/// # Type Parameters
///
/// - `Num`: A numeric type that implements [`egui::emath::Numeric`].
///
/// # Parameters
///
/// - `data`: A mutable reference to the numeric value to be modified by the slider.
/// - `label`: The label displayed next to the slider.
/// - `tooltip`: A short description shown as a tooltip when hovering over the label.
/// - `label_ratio`: The width percent for the label.
/// - `read_only`: If `true`, disables interaction with the slider.
/// - `min`: The minimum value of the slider range.
/// - `max`: The maximum value of the slider range.
/// - `ui`: The [`egui::Ui`] instance to which the slider will be added.
///
/// # See Also
///
/// - [`egui::Slider`]
/// - [`add_number`]
#[allow(clippy::too_many_arguments)]
pub fn add_number_slider<Num: egui::emath::Numeric>(
	data: &mut Num,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	min: Num,
	max: Num,
	ui: &mut egui::Ui,
) -> egui::Response {
	let editor = egui::Slider::new(data, min..=max);
	crate::add_custom_ui(
		label,
		tooltip,
		label_ratio,
		read_only,
		ui,
		|ui, field_width| {
			ui.spacing_mut().slider_width = field_width - 50.;
			ui.add_sized([field_width, 0.], editor)
		},
	)
}
/// Adds a numeric drag field to the UI.
///
/// - `data`: Mutable reference to the numeric value.
/// - `label`: Label shown next to the field.
/// - `tooltip`: Tooltip shown when hovering.
/// - `label_ratio`: The width percent for the label.
/// - `read_only`: If `true`, disables interaction.
/// - `minmax`: Optional `(min, max)` range.
/// - `ui`: The `egui::Ui` to render into.
///
/// See full documentation in [`add_number_slider`].
///
/// # See Also
///
/// - [`egui::DragValue`]
/// - [`add_number`]
pub fn add_number<Num: egui::emath::Numeric>(
	data: &mut Num,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	minmax: Option<(Num, Num)>,
	ui: &mut egui::Ui,
) -> egui::Response {
	let mut editor = egui::DragValue::new(data);
	if let Some(minmax) = minmax {
		editor = editor.range(minmax.0..=minmax.1);
	}
	crate::add_widget(label, editor, tooltip, label_ratio, read_only, ui)
}

/// Adds a single-line text field.
///
/// # See Also
///
/// - [`egui::TextEdit::singleline`]
pub fn add_string_singleline(
	data: &mut dyn egui::TextBuffer,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	crate::add_widget(
		label,
		egui::TextEdit::singleline(data),
		tooltip,
		label_ratio,
		read_only,
		ui,
	)
}

/// Adds a multi-line text field with a specified number of visible lines.
///
/// # See Also
///
/// - [`egui::TextEdit::multiline`]
pub fn add_string_multiline(
	data: &mut dyn egui::TextBuffer,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	nb_lines: u8,
	ui: &mut egui::Ui,
) -> egui::Response {
	crate::add_widget(
		label,
		egui::TextEdit::multiline(data).desired_rows(nb_lines as usize),
		tooltip,
		label_ratio,
		read_only,
		ui,
	)
}

/// Adds a boolean checkbox.
///
/// # See Also
///
/// - [`egui::Checkbox`]
pub fn add_bool(
	data: &mut bool,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	crate::add_widget(
		label,
		egui::Checkbox::new(data, ""),
		tooltip,
		label_ratio,
		read_only,
		ui,
	)
}

/// Adds a color picker for [`egui::Color32`].
///
/// # See Also
///
/// - [`egui::Ui::color_edit_button_srgba`]
pub fn add_color32(
	data: &mut egui::Color32,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	let label_ratio = label_ratio.clamp(0.05, 0.95);

	let available_width = ui.available_width();
	let label_width = available_width * label_ratio;
	//let field_width = 100.0f32.max(available_width * (1.0-label_ratio) - 10.0);
	ui.horizontal(|ui| {
		ui.add_enabled_ui(!read_only, |ui| {
			let r = ui.add_sized(
				[label_width, 0.],
				egui::Label::new(label)
					.truncate()
					.show_tooltip_when_elided(true)
					.halign(egui::Align::LEFT),
			);
			if !tooltip.is_empty() {
				if !read_only {
					r.on_hover_text(tooltip);
				} else {
					r.on_disabled_hover_text(tooltip);
				}
			}
		});
		ui.color_edit_button_srgba(data)
	})
	.inner
}

/// Adds a color picker for custom color types convertible to/from [`Color32Wrapper`].
///
/// # See Also
///
/// - [`egui::Ui::color_edit_button_srgba`]
pub fn add_color<T>(
	data: &mut T,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut egui::Ui,
) -> egui::Response
where
	Color32Wrapper: From<T>,
	T: From<Color32Wrapper>,
	T: Clone,
{
	crate::add_custom_ui(
		label,
		tooltip,
		label_ratio,
		read_only,
		ui,
		|ui, _field_width| {
			let mut color: Color32Wrapper = data.clone().into();
			let res = ui.color_edit_button_srgba(&mut color);
			if res.changed() {
				*data = color.into();
			}
			res
		},
	)
}

/// Adds a [egui::ComboBox] to modify the index of chosed in the `choices` array.
///
/// # Panics
/// When `current_index` is out of bounds of `choices`.
///
/// # See Also
///
/// - [egui::ComboBox]
pub fn add_combobox(
	current_index: &mut usize,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	choices: &[String],
	ui: &mut egui::Ui,
) -> egui::Response {
	//TODO: good management of id_salt
	crate::add_custom_ui(
		label,
		tooltip,
		label_ratio,
		read_only,
		ui,
		|ui, field_width| {
			egui::ComboBox::from_id_salt(label)
				.width(field_width)
				.show_index(ui, current_index, choices.len(), |i| &choices[i])
		},
	)
}
/// Add a [egui::Button]
pub fn add_button<F>(
	label: &str,
	tooltip: &str,
	read_only: bool,
	ui: &mut egui::Ui,
	on_click: F,
) -> egui::Response
where
	F: FnOnce(&mut egui::Ui),
{
	let button = egui::Button::new(label).min_size(egui::vec2(ui.available_width(), 0.));
	ui.add_enabled_ui(!read_only, |ui| {
		ui.horizontal_top(|ui| {
			let mut r = button.ui(ui);
			if !tooltip.is_empty() {
				r = r.on_hover_text(tooltip);
			}
			if r.clicked() {
				on_click(ui);
				r.mark_changed();
			}
			r
		})
		.inner
	})
	.inner
}
/// Add a single line text field which use string conversions to edit.
pub fn add_string_convertible<T>(
	value: &mut T,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut Ui,
) -> egui::Response
where
	T: FromStr + Display,
{
	let mut buffer = value.to_string();

	let r = buffer.inspect_with_custom_id(
		ui.next_auto_id().with(label),
		label,
		tooltip,
		label_ratio,
		read_only,
		ui,
	);

	if let Ok(parsed) = T::from_str(&buffer) {
		*value = parsed;
	} else {
		ui.label("❌ Invalid format");
	}
	r
}
/// Add a multiline line text field which use string conversions to edit.
pub fn add_string_convertible_multiline<T>(
	value: &mut T,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	ui: &mut Ui,
) -> egui::Response
where
	T: FromStr + Display,
{
	let mut buffer = value.to_string();

	let r = crate::add_string_multiline(&mut buffer, label, tooltip, label_ratio, read_only, 4, ui);

	if let Ok(parsed) = T::from_str(&buffer) {
		*value = parsed;
	} else {
		ui.label("❌ Invalid format");
	}
	r
}

/// An utility wrapper around [`jiff::civil::Date`].
///
/// This wrapper is useful when you want to:
/// - Implement custom traits or methods on top of `Date`
/// - Use `Deref` to access `Date` fields directly
/// - Maintain compatibility with `egui` while adding abstraction
///
/// # Trait Implementations
///
/// - [`Clone`], [`Copy`], [`Debug`] for ergonomic use
/// - [`From<Date>`] and [`From<DateWrapper>`] for conversion
/// - [`Deref`] and [`DerefMut`] to access `Color32` transparently
///
/// # See Also
///
/// - [`jiff::civil::Date`]
#[derive(Clone, Debug, Copy)]
#[cfg(feature = "datepicker")]
pub struct DateWrapper(jiff::civil::Date);
#[cfg(feature = "datepicker")]
impl From<jiff::civil::Date> for DateWrapper {
	fn from(value: jiff::civil::Date) -> Self {
		Self(value)
	}
}
#[cfg(feature = "datepicker")]
impl From<DateWrapper> for jiff::civil::Date {
	fn from(value: DateWrapper) -> Self {
		value.0
	}
}
#[cfg(feature = "datepicker")]
impl Deref for DateWrapper {
	type Target = jiff::civil::Date;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
#[cfg(feature = "datepicker")]
impl DerefMut for DateWrapper {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
#[cfg(feature = "datepicker")]
impl PartialEq for DateWrapper {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}
#[cfg(feature = "datepicker")]
impl Eq for DateWrapper {}

/// Adds a date picker for date types.
///
/// # Parameters
/// - `combo_boxes`: Show combo boxes in date picker popup. (Default: true)
/// - `arrows`: Show arrows in date picker popup. (Default: true)
/// - `calendar`: Show calendar in date picker popup. (Default: true)
/// - `calendar_week`: Show calendar week in date picker popup. (Default: true)
/// - `show_icon`: Show the calendar icon on the button. (Default: true)
/// - `format`: Change the format shown on the button. (Default: `"%Y-%m-%d"`)
///
///    See [`chrono::format::strftime`] for valid formats.
/// - `highlight_weekends`: Highlight weekend days. (Default: true)
/// - `start_end_years`: Set the start and end years for the date picker. (Default: today's year - 100 to today's year + 10)
///
///    This will limit the years you can choose from in the dropdown to the specified range.
///
/// # See Also
///
/// - [`egui_extras::DatePickerButton`]
#[cfg(feature = "datepicker")]
#[allow(clippy::too_many_arguments)] // TODO: find a better way to pass arguments
pub fn add_civil_date(
	data: &mut jiff::civil::Date,
	parent_id: egui::Id,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	combo_boxes: bool,
	arrows: bool,
	calendar: bool,
	calendar_week: bool,
	show_icon: bool,
	format: String,
	highlight_weekends: bool,
	start_end_years: Option<RangeInclusive<i16>>,
	ui: &mut egui::Ui,
) -> egui::Response {
	let id = if parent_id == egui::Id::NULL {
		egui::Id::NULL
	} else {
		parent_id.with(label)
	};

	let mut widget = egui_extras::DatePickerButton::new(data)
		.combo_boxes(combo_boxes)
		.arrows(arrows)
		.calendar(calendar)
		.calendar_week(calendar_week)
		.show_icon(show_icon)
		.format(format)
		.highlight_weekends(highlight_weekends);
	if let Some(start_end_years) = start_end_years {
		widget = widget.start_end_years(start_end_years);
	}
	let res = if id != egui::Id::NULL {
		// Ugly hack because DatePickerButton::id_salt() taking a &str
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
	res
}

/// Adds a date picker for date types.
///
/// # Parameters
/// - `combo_boxes`: Show combo boxes in date picker popup. (Default: true)
/// - `arrows`: Show arrows in date picker popup. (Default: true)
/// - `calendar`: Show calendar in date picker popup. (Default: true)
/// - `calendar_week`: Show calendar week in date picker popup. (Default: true)
/// - `show_icon`: Show the calendar icon on the button. (Default: true)
/// - `format`: Change the format shown on the button. (Default: `"%Y-%m-%d"`)
///
///    See [`chrono::format::strftime`] for valid formats.
/// - `highlight_weekends`: Highlight weekend days. (Default: true)
/// - `start_end_years`: Set the start and end years for the date picker. (Default: today's year - 100 to today's year + 10)
///
///    This will limit the years you can choose from in the dropdown to the specified range.
///
/// # See Also
///
/// - [`egui_extras::DatePickerButton`]
#[cfg(feature = "datepicker")]
#[allow(clippy::too_many_arguments)] // TODO: find a better way to pass arguments
pub fn add_date<T>(
	data: &mut T,
	parent_id: egui::Id,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	combo_boxes: bool,
	arrows: bool,
	calendar: bool,
	calendar_week: bool,
	show_icon: bool,
	format: String,
	highlight_weekends: bool,
	start_end_years: Option<RangeInclusive<i16>>,
	ui: &mut egui::Ui,
) -> egui::Response
where
	DateWrapper: From<T>,
	T: From<DateWrapper>,
	T: Clone,
{
	let mut date: DateWrapper = data.clone().into();
	let res = add_civil_date(
		&mut date,
		parent_id,
		label,
		tooltip,
		label_ratio,
		read_only,
		combo_boxes,
		arrows,
		calendar,
		calendar_week,
		show_icon,
		format,
		highlight_weekends,
		start_end_years,
		ui,
	);
	if res.changed() {
		*data = date.into();
	}
	res
}

/// Add a path (a singleline string editor) with a button next to it to open a file picker if the feature "filepicker" is active
pub fn add_path(
	data: &mut std::path::PathBuf,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	_filters: Vec<&str>,
	ui: &mut egui::Ui,
) -> egui::Response {
	add_custom_ui(
		label,
		tooltip,
		label_ratio,
		read_only,
		ui,
		|ui, field_width| {
			if let Some(path) = data.to_str() {
				let mut path = path.to_string();
				#[cfg(all(feature = "filepicker", not(target_arch = "wasm32")))]
				let field_width = if !read_only {
					field_width - 35.
				} else {
					field_width
				};

				let res = ui.add_enabled(
					!read_only,
					egui::TextEdit::singleline(&mut path).desired_width(field_width),
				);
				if res.changed() {
					*data = path.into();
				}
				#[cfg(all(feature = "filepicker", not(target_arch = "wasm32")))]
				let res = if !read_only {
					let mut btn_res = ui.button("...");
					if btn_res.clicked() {
						let mut fd = rfd::FileDialog::new();
						for f in &_filters {
							fd = fd.add_filter(f.to_string(), &f.split(',').collect::<Vec<_>>());
						}
						if !_filters.is_empty() {
							fd = fd.add_filter("All Files".to_string(), &["*.*"]);
						}
						let filepath = fd.pick_file();
						if let Some(filepath) = filepath {
							*data = filepath;
							btn_res.mark_changed();
						}
					}
					res.union(btn_res)
				} else {
					res
				};
				res
			} else {
				ui.response()
			}
		},
	)
}

/// An utility wrapper around [`egui::Color32`].
///
/// This wrapper is useful when you want to:
/// - Implement custom traits or methods on top of `Color32`
/// - Use `Deref` to access `Color32` fields directly
/// - Maintain compatibility with `egui` while adding abstraction
///
/// # Trait Implementations
///
/// - [`Clone`], [`Copy`], [`Default`], [`Debug`] for ergonomic use
/// - [`From<Color32>`] and [`From<Color32Wrapper>`] for conversion
/// - [`Deref`] and [`DerefMut`] to access `Color32` transparently
///
/// # See Also
///
/// - [`egui::Color32`]
#[derive(Clone, Copy, Debug, Default)]
pub struct Color32Wrapper(egui::Color32);
impl From<Color32> for Color32Wrapper {
	fn from(value: Color32) -> Self {
		Self(value)
	}
}
impl From<Color32Wrapper> for Color32 {
	fn from(value: Color32Wrapper) -> Self {
		value.0
	}
}
impl Deref for Color32Wrapper {
	type Target = egui::Color32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for Color32Wrapper {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl PartialEq for Color32Wrapper {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}
impl Eq for Color32Wrapper {}

#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2, Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3, Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4, Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_dvec2, DVec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_dvec3, DVec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_dvec4, DVec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2u8, U8Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3u8, U8Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4u8, U8Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2i8, I8Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3i8, I8Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4i8, I8Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2u16, U16Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3u16, U16Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4u16, U16Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2i16, I16Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3i16, I16Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4i16, I16Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2u32, U32Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3u32, U32Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4u32, U32Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2i32, I32Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3i32, I32Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4i32, I32Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2u64, U64Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3u64, U64Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4u64, U64Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec2i64, I64Vec2, [x, y]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec3i64, I64Vec3, [x, y, z]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_vec4i64, I64Vec4, [x, y, z, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_quat, Quat, [i, j, k, w]);
#[cfg(feature = "nalgebra_glm")]
impl_only_numbers_struct_inspect!(add_dquat, DQuat, [i, j, k, w]);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(add_mat2x2, Mat2x2, [[m11, m12], [m21, m22]]);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(add_mat2x3, Mat2x3, [[m11, m12, m13], [m21, m22, m23]]);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(
	add_mat2x4,
	Mat2x4,
	[[m11, m12, m13, m14], [m21, m22, m23, m24]]
);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(add_mat3x2, Mat3x2, [[m11, m12], [m21, m22], [m31, m32]]);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(
	add_mat3x3,
	Mat3x3,
	[[m11, m12, m13], [m21, m22, m23], [m31, m32, m33]]
);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(
	add_mat3x4,
	Mat3x4,
	[
		[m11, m12, m13, m14],
		[m21, m22, m23, m24],
		[m31, m32, m33, m34]
	]
);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(
	add_mat4x2,
	Mat4x2,
	[[m11, m12], [m21, m22], [m31, m32], [m41, m42]]
);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(
	add_mat4x3,
	Mat4x3,
	[
		[m11, m12, m13],
		[m21, m22, m23],
		[m31, m32, m33],
		[m41, m42, m43]
	]
);
#[cfg(feature = "nalgebra_glm")]
impl_mat_inspect!(
	add_mat4x4,
	Mat4x4,
	[
		[m11, m12, m13, m14],
		[m21, m22, m23, m24],
		[m31, m32, m33, m34],
		[m41, m42, m43, m44]
	]
);

/// Display and edit a HashMap<String, T> with configurable behavior
///
/// # Parameters
/// - `data`: The HashMap to inspect
/// - `parent_id`: Parent egui ID
/// - `label`: Label for the field
/// - `tooltip`: Tooltip text
/// - `label_ratio`: Ratio for label width
/// - `read_only`: If true, disables all editing
/// - `allow_add`: Show button to add new entries
/// - `allow_delete`: Show button to delete entries
/// - `editable_keys`: Allow editing the string keys
/// - `ui`: The egui Ui context
#[allow(clippy::too_many_arguments)]
pub fn add_hashmap<T: EguiInspect + Default + Clone>(
	data: &mut std::collections::HashMap<String, T>,
	parent_id: egui::Id,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	allow_add_delete: bool,
	editable_keys: bool,
	ui: &mut egui::Ui,
) -> egui::Response {
	let id = if parent_id == egui::Id::NULL {
		ui.next_auto_id()
	} else {
		parent_id.with(label)
	};
	let mut changed = false;
	let data_len = data.len();
	let mut add_content = |ui: &mut Ui| {
		let keys: Vec<String> = data.keys().cloned().collect();
		let mut resp = ui.response();

		for key in keys {
			if let Some(mut value) = data.remove(&key) {
				let mut edited_key = key.clone();

				let inner_res = if editable_keys {
					ui.horizontal_top(|ui| {
						ui.add_enabled_ui(!read_only, |ui| {
							let mut te = edited_key.clone();
							let res = ui.add_sized(
								[ui.available_width() * label_ratio, 0.0],
								egui::TextEdit::singleline(&mut te),
							);

							if res.changed() && te != key {
								edited_key = te.clone();
								changed = true;
							}

							let value_res = ui
								.vertical(|ui| {
									value.inspect_with_custom_id(
										id.with(&edited_key),
										"",
										tooltip,
										0.0,
										read_only,
										ui,
									)
								})
								.inner;

							res.union(value_res)
						})
					})
				} else {
					ui.horizontal_top(|ui| {
						ui.add_enabled_ui(!read_only, |ui| {
							ui.vertical(|ui| {
								value.inspect_with_custom_id(
									id.with(&edited_key),
									&key,
									tooltip,
									label_ratio,
									read_only,
									ui,
								)
							})
							.inner
						})
					})
				};

				data.insert(edited_key, value);
				resp = resp.union(inner_res.inner.inner);
			}
		}

		resp
	};
	let content_resp = if !label.is_empty() {
		egui::CollapsingHeader::new(format!("{label} [{}]", data_len))
			.id_salt(id.with("collapse"))
			.show(ui, add_content)
			.body_returned
	} else {
		Some(add_content(ui))
	};
	if allow_add_delete {
		ui.add_enabled_ui(!read_only, |ui| {
			ui.horizontal_top(|ui| {
				ui.add_space(ui.available_width() - 50.0);

				if ui
					.add(egui::Button::new("+").min_size(egui::Vec2::new(20., 20.)))
					.clicked()
				{
					let mut i = 0;
					while data.contains_key(&i.to_string()) {
						i += 1;
					}
					data.insert(i.to_string(), T::default());
					changed = true;
				}
				#[allow(clippy::collapsible_if)]
				if ui
					.add(egui::Button::new("-").min_size(egui::Vec2::new(20., 20.)))
					.clicked()
				{
					if let Some(last_key) = data.keys().last().cloned() {
						data.remove(&last_key);
						changed = true;
					}
				}
			});
		});
	}
	let mut final_res = ui.response();
	if let Some(body_res) = content_resp {
		final_res = final_res.union(body_res);
	}
	if changed {
		final_res.mark_changed();
	}

	final_res
}

/// Display and edit a HashMap<String, T> with configurable behavior and custom value inspector
///
/// This variant allows you to provide a custom function for inspecting values.
///
/// # Parameters
/// - `data`: The HashMap to inspect
/// - `parent_id`: Parent egui ID
/// - `label`: Label for the field
/// - `tooltip`: Tooltip text
/// - `label_ratio`: Ratio for label width
/// - `read_only`: If true, disables all editing
/// - `allow_add_delete`: Show button to add/delete entries
/// - `editable_keys`: Allow editing the string keys
/// - `custom_fn`: Custom function to inspect values: `fn(value: &mut T, parent_id: Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut Ui) -> Response`
/// - `ui`: The egui Ui context
#[allow(clippy::too_many_arguments)]
pub fn add_hashmap_custom<T, F>(
	data: &mut std::collections::HashMap<String, T>,
	parent_id: egui::Id,
	label: &str,
	tooltip: &str,
	label_ratio: f32,
	read_only: bool,
	allow_add_delete: bool,
	editable_keys: bool,
	custom_fn: F,
	ui: &mut egui::Ui,
) -> egui::Response
where
	T: Default + Clone,
	F: Fn(&mut T, egui::Id, &str, &str, f32, bool, &mut egui::Ui) -> egui::Response,
{
	let id = if parent_id == egui::Id::NULL {
		ui.next_auto_id()
	} else {
		parent_id.with(label)
	};
	let mut changed = false;
	let data_len = data.len();
	let mut add_content = |ui: &mut Ui| {
		let keys: Vec<String> = data.keys().cloned().collect();
		let mut resp = ui.response();

		for key in keys {
			if let Some(mut value) = data.remove(&key) {
				let mut edited_key = key.clone();

				let inner_res = if editable_keys {
					ui.horizontal_top(|ui| {
						ui.add_enabled_ui(!read_only, |ui| {
							let mut te = edited_key.clone();
							let res = ui.add_sized(
								[ui.available_width() * label_ratio, 0.0],
								egui::TextEdit::singleline(&mut te),
							);

							if res.changed() && te != key {
								edited_key = te.clone();
								changed = true;
							}

							let value_res = ui
								.vertical(|ui| {
									custom_fn(
										&mut value,
										id.with(&edited_key),
										"",
										tooltip,
										0.0,
										read_only,
										ui,
									)
								})
								.inner;

							res.union(value_res)
						})
					})
				} else {
					ui.horizontal_top(|ui| {
						ui.add_enabled_ui(!read_only, |ui| {
							custom_fn(
								&mut value,
								id.with(&edited_key),
								&key,
								tooltip,
								label_ratio,
								read_only,
								ui,
							)
						})
					})
				};

				data.insert(edited_key, value);
				resp = resp.union(inner_res.inner.inner);
			}
		}

		resp
	};

	let content_resp = if !label.is_empty() {
		egui::CollapsingHeader::new(format!("{label} [{}]", data_len))
			.id_salt(id.with("collapse"))
			.show(ui, add_content)
			.body_returned
	} else {
		Some(add_content(ui))
	};

	// Add/Delete buttons - only shown if operations are allowed
	if allow_add_delete {
		ui.add_enabled_ui(!read_only, |ui| {
			ui.horizontal_top(|ui| {
				ui.add_space(ui.available_width() - 50.0);

				if ui
					.add(egui::Button::new("+").min_size(egui::Vec2::new(20., 20.)))
					.clicked()
				{
					let mut i = 0;
					while data.contains_key(&i.to_string()) {
						i += 1;
					}
					data.insert(i.to_string(), T::default());
					changed = true;
				}
				#[allow(clippy::collapsible_if)]
				if ui
					.add(egui::Button::new("-").min_size(egui::Vec2::new(20., 20.)))
					.clicked()
				{
					if let Some(last_key) = data.keys().last().cloned() {
						data.remove(&last_key);
						changed = true;
					}
				}
			});
		});
	}

	let mut final_res = ui.response();
	if let Some(body_res) = content_resp {
		final_res = final_res.union(body_res);
	}
	if changed {
		final_res.mark_changed();
	}

	final_res
}

mod base_type_inspect;
#[allow(missing_docs)]
pub mod collapsing_enum_variant_editor;
