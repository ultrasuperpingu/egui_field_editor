use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Field;
use syn::Type;
use syn::spanned::Spanned;

use crate::AttributeArgs;

#[allow(dead_code)]
pub fn get_path_str(type_path: &Type) -> String {
	quote::ToTokens::to_token_stream(&type_path).to_string()
}

pub fn prettify_name(s: &str) -> String {
	s.split('_')
		.filter(|part| !part.is_empty())
		.map(|word| {
			let split_index = word
				.char_indices()
				.rev()
				.find(|&(_, c)| !c.is_ascii_digit())
				.map(|(i, _)| i + 1)
				.unwrap_or(0);

			let (letters, digits) = word.split_at(split_index);

			let mut chars = letters.chars();
			let capitalized = match chars.next() {
				Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
				None => String::new(),
			};

			if digits.is_empty() {
				capitalized
			} else {
				format!("{capitalized} {digits}")
			}
		})
		.collect::<Vec<_>>()
		.join(" ")
}

pub(crate) fn get_function_call(
	field_access: TokenStream,
	field: &Field,
	attrs: &AttributeArgs,
	default_field_name: String,
) -> TokenStream {
	let name = &field.ident;

	let name_str = if attrs.transparent {
		"".into()
	} else {
		match &attrs.name {
			Some(n) => n.clone(),
			None => {
				if let Some(name) = name {
					prettify_name(&name.to_string())
				} else {
					prettify_name(&default_field_name.to_string())
				}
			}
		}
	};
	let read_only = attrs.read_only;
	let slider = &attrs.slider;
	let range = &attrs.range;
	let mut tooltip = "";
	if let Some(ttip) = attrs.tooltip.as_ref() {
		tooltip = ttip;
	}
	if let Some(custom_fn) = &attrs.custom_fn {
		match custom_fn.parse::<TokenStream>() {
			Ok(custom_fn_ident) => {
				return quote_spanned! {
					field.span() => {
						egui_field_editor::combine_responses!(ui.scope(|ui| {
							#custom_fn_ident(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, ui)
						}).inner, res);
					}
				};
			}
			Err(e) => {
				let msg = e.to_string();
				return quote_spanned! {
					field.span() => {
						compile_error!(#msg);
					}
				};
			}
		}
	} else if let Some(range) = slider {
		let min = range.min;
		let max = range.max;
		let ty = proc_macro2::Ident::new(&get_path_str(&field.ty), proc_macro2::Span::call_site());
		return quote_spanned! {
			field.span() => {
				egui_field_editor::combine_responses!(ui.scope(|ui| {
					egui_field_editor::add_number_slider(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, #min as #ty, #max as #ty, ui)
				}).inner, res);
			}
		};
	} else if let Some(range) = range {
		let min = range.min;
		let max = range.max;
		let ty = proc_macro2::Ident::new(&get_path_str(&field.ty), proc_macro2::Span::call_site());
		return quote_spanned! {field.span() => {
				egui_field_editor::combine_responses!(ui.scope(|ui| {
					egui_field_editor::add_number(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, Some((#min as #ty, #max as #ty)), ui)
				}).inner, res);
			}
		};
	} else if attrs.from_string {
		if let Some(multiline) = &attrs.multiline {
			let nb_lines = multiline.0;
			return quote_spanned! {
				field.span() => {
					egui_field_editor::combine_responses!(ui.scope(|ui| {
						egui_field_editor::add_string_convertible_multiline(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, #nb_lines, ui)
					}).inner, res);
				}
			};
		} else {
			return quote_spanned! {
				field.span() => {
					egui_field_editor::combine_responses!(ui.scope(|ui| {
						egui_field_editor::add_string_convertible(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, ui)
					}).inner, res);
				}
			};
		}
	} else if let Some(multiline) = &attrs.multiline {
		let nb_lines = multiline.0;
		return quote_spanned! {
			field.span() => {
				egui_field_editor::combine_responses!(ui.scope(|ui| {
					egui_field_editor::add_string_multiline(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, #nb_lines, ui)
				}).inner, res);
			}
		};
	} else if attrs.color {
		return quote_spanned! {field.span() => {
				egui_field_editor::combine_responses!(ui.scope(|ui| {
					egui_field_editor::add_color(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, ui)
				}).inner, res);
			}
		};
	} else if let Some(file) = &attrs.file {
		let filters: Vec<proc_macro2::TokenStream> = file
			.filter
			.iter()
			.map(|e| {
				let lit = syn::LitStr::new(e, proc_macro2::Span::call_site());
				quote! { #lit }
			})
			.collect();
		return quote_spanned! {field.span() => {
				egui_field_editor::combine_responses!(ui.scope(|ui| {
					egui_field_editor::add_path(#field_access, &#name_str, #tooltip, label_ratio, read_only || #read_only, vec![#(#filters),*], ui)
				}).inner, res);
			}
		};
	} else if let Some(date) = &attrs.date {
		let combo_boxes = date.combo_boxes;
		let arrows = date.arrows;
		let calendar = date.calendar;
		let calendar_week = date.calendar_week;
		let show_icon = date.show_icon;
		let format = date.format.to_owned();
		let highlight_weekends = date.highlight_weekends;
		let mut start_end_years = quote_spanned! {field.span() => {None}};
		if let Some(range) = &date.start_end_years {
			let min = range.min;
			let max = range.max;
			start_end_years = quote_spanned! {field.span() => {Some(#min..=#max)}};
		}
		return quote_spanned! {field.span() => {
				egui_field_editor::combine_responses!(ui.scope(|ui| {
					egui_field_editor::add_date(#field_access, id, &#name_str, #tooltip, label_ratio, read_only || #read_only,
						#combo_boxes,
						#arrows,
						#calendar,
						#calendar_week,
						#show_icon,
						#format.to_string(),
						#highlight_weekends,
						#start_end_years,
						ui)
				}).inner, res);
			}
		};
	} else if let Some(hmap_opts) = &attrs.hashmap {
		let allow_add_delete = hmap_opts.allow_add_delete;
		let editable_keys = hmap_opts.editable_keys;

		if let Some(custom_fn) = &hmap_opts.custom_fn {
			match custom_fn.parse::<TokenStream>() {
				Ok(custom_fn_ident) => {
					return quote_spanned! {
						field.span() => {
							let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
							let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
							egui_field_editor::combine_responses!(ui.scope(|ui| {
								egui_field_editor::add_hashmap_custom(
									#field_access,
									parent_id,
									&#name_str,
									#tooltip,
									label_ratio,
									read_only || #read_only,
									#allow_add_delete,
									#editable_keys,
									|value, pid, lbl, tp, lr, ro, u| { #custom_fn_ident(value, pid, lbl, tp, lr, ro, u) },
									ui
								)
							}).inner, res);
						}
					};
				}
				Err(e) => {
					let msg = e.to_string();
					return quote_spanned! {
						field.span() => {
							compile_error!(#msg);
						}
					};
				}
			}
		} else {
			return quote_spanned! {
				field.span() => {
					let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
					let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
					egui_field_editor::combine_responses!(ui.scope(|ui| {
						egui_field_editor::add_hashmap(
							#field_access,
							parent_id,
							&#name_str,
							#tooltip,
							label_ratio,
							read_only || #read_only,
							#allow_add_delete,
							#editable_keys,
							ui
						)
					}).inner, res);
				}
			};
		}
	}

	quote_spanned! {
		field.span() => {
			let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
			let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
			egui_field_editor::combine_responses!(ui.scope(|ui| {
				egui_field_editor::EguiInspect::inspect_with_custom_id(#field_access, parent_id, &#name_str, #tooltip, label_ratio, read_only || #read_only, ui)
			}).inner, res);
		}
	}
}
