#![allow(unused)]
use egui::{
	Id, Response, Sense, Ui, WidgetText,
	collapsing_header::{CollapsingState, IconPainter, paint_default_icon},
};
use std::hash::Hash;

pub struct CollapsingEnumVariantEditor {
	text: WidgetText,
	tooltip: WidgetText,
	label_ratio: f32,
	default_open: bool,
	open: Option<bool>,
	id_salt: Id,
	enabled: bool,
	selectable: bool,
	selected: bool,
	show_background: bool,
	icon: Option<IconPainter>,
}

impl CollapsingEnumVariantEditor {
	/// The [`CollapsingHeader`] starts out collapsed unless you call `default_open`.
	///
	/// The label is used as an [`Id`] source.
	/// If the label is unique and static this is fine,
	/// but if it changes or there are several [`CollapsingHeader`] with the same title
	/// you need to provide a unique id source with [`Self::id_salt`].
	pub fn new(
		text: impl Into<WidgetText>,
		tooltip: impl Into<WidgetText>,
		label_ratio: f32,
	) -> Self {
		let text = text.into();
		let tooltip = tooltip.into();
		let id_salt = Id::new(text.text());
		Self {
			text,
			tooltip,
			label_ratio,
			default_open: false,
			open: None,
			id_salt,
			enabled: true,
			selectable: false,
			selected: false,
			show_background: false,
			icon: None,
		}
	}

	/// By default, the [`CollapsingHeader`] is collapsed.
	/// Call `.default_open(true)` to change this.
	#[inline]
	pub fn default_open(mut self, open: bool) -> Self {
		self.default_open = open;
		self
	}

	/// Calling `.open(Some(true))` will make the collapsing header open this frame (or stay open).
	///
	/// Calling `.open(Some(false))` will make the collapsing header close this frame (or stay closed).
	///
	/// Calling `.open(None)` has no effect (default).
	#[inline]
	pub fn open(mut self, open: Option<bool>) -> Self {
		self.open = open;
		self
	}

	/// Explicitly set the source of the [`Id`] of this widget, instead of using title label.
	/// This is useful if the title label is dynamic or not unique.
	#[inline]
	pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
		self.id_salt = Id::new(id_salt);
		self
	}

	/// Explicitly set the source of the [`Id`] of this widget, instead of using title label.
	/// This is useful if the title label is dynamic or not unique.
	#[deprecated = "Renamed id_salt"]
	#[inline]
	pub fn id_source(mut self, id_salt: impl Hash) -> Self {
		self.id_salt = Id::new(id_salt);
		self
	}

	/// If you set this to `false`, the [`CollapsingHeader`] will be grayed out and un-clickable.
	///
	/// This is a convenience for [`Ui::disable`].
	#[inline]
	pub fn enabled(mut self, enabled: bool) -> Self {
		self.enabled = enabled;
		self
	}

	/// Should the [`CollapsingHeader`] show a background behind it? Default: `false`.
	///
	/// To show it behind all [`CollapsingHeader`] you can just use:
	/// ```
	/// # egui::__run_test_ui(|ui| {
	/// ui.visuals_mut().collapsing_header_frame = true;
	/// # });
	/// ```
	#[inline]
	pub fn show_background(mut self, show_background: bool) -> Self {
		self.show_background = show_background;
		self
	}

	/// Use the provided function to render a different [`CollapsingHeader`] icon.
	/// Defaults to a triangle that animates as the [`CollapsingHeader`] opens and closes.
	///
	/// For example:
	/// ```
	/// # egui::__run_test_ui(|ui| {
	/// fn circle_icon(ui: &mut egui::Ui, openness: f32, response: &egui::Response) {
	///     let stroke = ui.style().interact(&response).fg_stroke;
	///     let radius = egui::lerp(2.0..=3.0, openness);
	///     ui.painter().circle_filled(response.rect.center(), radius, stroke.color);
	/// }
	///
	/// egui::CollapsingHeader::new("Circles")
	///   .icon(circle_icon)
	///   .show(ui, |ui| { ui.label("Hi!"); });
	/// # });
	/// ```
	#[inline]
	pub fn icon(mut self, icon_fn: impl FnOnce(&mut Ui, f32, &Response) + 'static) -> Self {
		self.icon = Some(Box::new(icon_fn));
		self
	}
}

struct Prepared {
	header_response: Response,
	combo_response: Option<Response>,
	state: CollapsingState,
	openness: f32,
}

impl CollapsingEnumVariantEditor {
	#[inline]
	pub fn show<R, Obj>(
		self,
		ui: &mut Ui,
		edited_obj: &mut Obj,
		selected_text: impl Into<WidgetText>,
		combo_content: impl FnOnce(&mut Ui, &mut Obj) -> Response,
		add_body: impl FnOnce(&mut Ui, &mut Obj) -> R,
		indented: bool,
	) -> CollapsingComboBoxResponse<R> {
		// Make sure body is bellow header,
		// and make sure it is one unit (necessary for putting a [`CollapsingHeader`] in a grid).
		ui.vertical(|ui| {
			if !self.enabled {
				ui.disable();
			}

			let Prepared {
				header_response,
				combo_response,
				mut state,
				openness,
			} = self.begin(ui, edited_obj, selected_text, combo_content); // show the header

			let ret_response = if indented {
				state.show_body_indented(&header_response, ui, |ui| add_body(ui, edited_obj))
			} else {
				state.show_body_unindented(ui, |ui| add_body(ui, edited_obj))
			};

			if let Some(ret_response) = ret_response {
				CollapsingComboBoxResponse {
					header_response,
					combo_response,
					body_response: Some(ret_response.response),
					body_returned: Some(ret_response.inner),
					openness,
				}
			} else {
				CollapsingComboBoxResponse {
					header_response,
					combo_response,
					body_response: None,
					body_returned: None,
					openness,
				}
			}
		})
		.inner
	}
	fn begin<Obj>(
		self,
		ui: &mut Ui,
		edited_obj: &mut Obj,
		selected_text: impl Into<WidgetText>,
		combo_content: impl FnOnce(&mut Ui, &mut Obj) -> Response,
	) -> Prepared {
		let Self {
			label_ratio,
			icon,
			text,
			default_open,
			open,
			id_salt,
			enabled: _,
			selectable: _,
			selected: _,
			show_background: _,
			tooltip,
		} = self;

		let id = ui.make_persistent_id(id_salt);

		let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, default_open);

		let (mut header_response, combo_response) = ui
			.horizontal(|ui| {
				let total_width = ui.available_width();
				let spacing = ui.spacing().item_spacing.x;

				let label_width = total_width * label_ratio;
				let combo_width = total_width - label_width - spacing;

				let (rect, _) = ui.allocate_exact_size(
					egui::vec2(label_width, ui.spacing().interact_size.y),
					Sense::click(),
				);

				let mut child_ui = ui.new_child(
					egui::UiBuilder::new()
						.max_rect(rect)
						.layout(egui::Layout::left_to_right(egui::Align::LEFT)),
				);

				let (icon_rect, _) = ui.spacing().icon_rectangles(rect);
				let (_rect, icon_resp) =
					child_ui.allocate_exact_size(icon_rect.size(), Sense::click());

				let openness = state.openness(ui.ctx());

				if let Some(icon) = icon {
					icon(&mut child_ui, openness, &icon_resp)
				} else {
					paint_default_icon(ui, openness, &icon_resp)
				}

				let mut label_response = child_ui
					.add(
						egui::Label::new(text.clone())
							.truncate()
							.halign(egui::Align::Min)
							.show_tooltip_when_elided(true),
					)
					.union(icon_resp);
				if !tooltip.is_empty() {
					if self.enabled {
						label_response = label_response.on_hover_text(tooltip);
					} else {
						label_response = label_response.on_disabled_hover_text(tooltip);
					}
				}

				let mut combo_response = ui
					.allocate_ui_with_layout(
						egui::vec2(combo_width, ui.spacing().interact_size.y),
						egui::Layout::right_to_left(egui::Align::Center),
						|ui| {
							egui::ComboBox::from_id_salt(id.with("combo"))
								.selected_text(selected_text)
								.width(combo_width)
								.truncate()
								.show_ui(ui, |ui| combo_content(ui, edited_obj))
								.inner
						},
					)
					.inner;
				(label_response, combo_response)
			})
			.inner;

		if let Some(open) = open {
			if open != state.is_open() {
				state.toggle(ui);
				header_response.mark_changed();
			}
		} else if header_response.clicked() {
			state.toggle(ui);
			header_response.mark_changed();
		}

		let openness = state.openness(ui.ctx());

		Prepared {
			header_response,
			combo_response,
			state,
			openness,
		}
	}
	#[inline]
	pub fn show_unindented<R, Obj>(
		self,
		ui: &mut Ui,
		edited_obj: &mut Obj,
		selected_text: &String,
		combo_content: impl FnOnce(&mut Ui, &mut Obj) -> Response,
		add_body: impl FnOnce(&mut Ui, &mut Obj) -> R,
	) -> CollapsingComboBoxResponse<R> {
		self.show(
			ui,
			edited_obj,
			selected_text,
			combo_content,
			add_body,
			false,
		)
	}
}

/// The response from showing a [`CollapsingHeader`].
pub struct CollapsingComboBoxResponse<R> {
	/// Response of the actual clickable header.
	pub header_response: Response,

	pub combo_response: Option<Response>,

	/// None iff collapsed.
	pub body_response: Option<Response>,

	/// None iff collapsed.
	pub body_returned: Option<R>,

	/// 0.0 if fully closed, 1.0 if fully open, and something in-between while animating.
	pub openness: f32,
}

impl<R> CollapsingComboBoxResponse<R> {
	/// Was the [`CollapsingHeader`] fully closed (and not being animated)?
	pub fn fully_closed(&self) -> bool {
		self.openness <= 0.0
	}

	/// Was the [`CollapsingHeader`] fully open (and not being animated)?
	pub fn fully_open(&self) -> bool {
		self.openness >= 1.0
	}
}
