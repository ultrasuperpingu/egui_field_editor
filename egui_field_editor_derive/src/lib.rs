#![forbid(unsafe_code)] 
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Index, LitStr, Meta
};

use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};

mod utils;

#[derive(Debug, FromMeta)]
struct FilePathParams {
	#[darling(multiple, default)]
	pub filter: Vec<String>
}
#[derive(Debug, FromMeta)]
#[darling(default)]
struct DatePickerParams {
	/// Show combo boxes in date picker popup. (Default: true)
	combo_boxes: bool,
	/// Show arrows in date picker popup. (Default: true)
	arrows: bool,
	/// Show calendar in date picker popup. (Default: true)
	calendar: bool,
	/// Show calendar week in date picker popup. (Default: true)
	calendar_week: bool,
	/// Show the calendar icon on the button. (Default: true)
	show_icon: bool,
	/// Change the format shown on the button. (Default: %Y-%m-%d)
	/// See [`chrono::format::strftime`] for valid formats.
	format: String,
	/// Highlight weekend days. (Default: true)
	highlight_weekends: bool,
	/// Set the start and end years for the date picker. (Default: today's year - 100 to today's year + 10)
	/// This will limit the years you can choose from in the dropdown to the specified range.
	///
	/// For example, if you want to provide the range of years from 2000 to 2035, you can use:
	/// `start_end_years(min=2000, max=2035)`.
	start_end_years: Option<Range<i16>>,
}
impl Default for DatePickerParams {
	fn default() -> Self {
		Self { combo_boxes: true, arrows: true, calendar: true, calendar_week: true, show_icon: true, format: "%Y-%m-%d".to_owned(), highlight_weekends: true, start_end_years: Default::default() }
	}
}
#[derive(Debug, Clone, FromMeta)]
struct Range<T : Default> {
	#[darling(default)]
	min: T,
	#[darling(default)]
	max: T,
}
#[derive(Debug, Default)]
struct Multiline(pub Option<u8>);

impl FromMeta for Multiline {
	fn from_meta(meta: &Meta) -> darling::Result<Self> {
		match meta {
			Meta::Path(_) => Ok(Multiline(Some(4))),
			Meta::NameValue(nv) => {
				let value = u8::from_expr(&nv.value)?;
				Ok(Multiline(Some(value)))
			}
			Meta::List(list) => {
				if list.tokens.is_empty() {
					Ok(Multiline(Some(4)))
				} else {
					let lit: syn::LitInt = syn::parse2(list.tokens.clone())
						.map_err(|e| darling::Error::custom(format!("Failed to parse list tokens: {e}")))?;
					let value = lit.base10_parse::<u8>()
						.map_err(|e| darling::Error::custom(format!("Invalid u8 value: {e}")))?;
					Ok(Multiline(Some(value)))
				}
			}
		}
	}
}
#[derive(Debug, FromMeta)]
struct ExecuteBtn {
	fn_name: LitStr,
	#[darling(default="bool_true")]
	is_method: bool,
	label: Option<LitStr>,
	tooltip: Option<LitStr>,
}
fn bool_true() -> bool {
	true
}
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
struct HashMapOptions {
	/// Allow adding new entries (default: false)
	allow_add_delete: bool,
	/// Allow editing keys (default: false)
	editable_keys: bool,
	/// Custom function to inspect values: fn(value: &mut T, parent_id: Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut Ui) -> Response
	custom_fn: Option<String>,
}
#[derive(Debug, Default, FromDeriveInput)]
#[darling(attributes(inspect), default)]
struct ObjectAttributeArgs {
	#[darling(multiple)]
	execute_btn: Vec<ExecuteBtn>,
	label_formatter: Option<syn::Expr>
}
#[derive(Debug, FromField, FromVariant, Default)]
#[darling(attributes(inspect), default)]
struct AttributeArgs {
	/// Name of the field to be displayed on UI labels
	name: Option<String>,
	/// Doesn't generate code for the given field
	hidden: bool,
	/// Display the field as readonly
	read_only: bool,
	/// Don't display a label for this field and don't indent (child fields will be display as they were directly field of the parent struct).
	transparent: bool,
	/// Use slider function for numbers
	slider: Option<Range<f32>>,
	/// Display text on multiple line
	multiline: Option<Multiline>,
	/// Display mut vec3/vec4 with color
	color: bool,
	/// Min/Max values for numbers
	range: Option<Range<f32>>,
	/// Tooltip for the field
	tooltip: Option<String>,
	/// Date picker options
	date: Option<DatePickerParams>,
	/// Date picker options
	file: Option<FilePathParams>,
	/// Force edition from string conversion (needs type to implement FromString and Display)
	from_string: bool,
	/// Use a custom function instead of calling [`EguiInspect::inspect_with_custom_id`]
	custom_fn: Option<String>,
	/// HashMap configuration options (allow_add_delete, editable_keys)
	hashmap: Option<HashMapOptions>,
}

#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn derive_egui_field_editor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let attrs= match ObjectAttributeArgs::from_derive_input(&input) {
			Ok(_attrs) => {
				_attrs
			}
			Err(e) => {
				let msg = e.to_string();
				return proc_macro::TokenStream::from(quote_spanned! {
					e.span() => {
						compile_error!(#msg);
					}
				});
			}
		};
	let exec_code = get_code_execute_btns(&attrs.execute_btn);
	let name = input.ident;

	let generics = add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let mut inspect_code = get_code_for_data(&input.data, &name);
	if !inspect_code.is_empty() {
		inspect_code = quote! {let resp=#inspect_code;}
	} else {
		inspect_code = quote! {let resp=ui.response();}
	}

	let expanded = quote! {
		impl #impl_generics egui_field_editor::EguiInspect for #name #ty_generics #where_clause {
			fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, label_ratio: f32, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
				let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
				let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
				#inspect_code
				#exec_code
				resp
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param
				.bounds
				.push(parse_quote!(egui_field_editor::EguiInspect));
		}
	}
	generics
}
fn get_code_execute_btns(execs: &[ExecuteBtn]) -> TokenStream {
	let recurse = execs.iter().map(|exec_fn| {
		let label = if let Some(l) = &exec_fn.label {
			l.value()
		} else {
			utils::prettify_name(exec_fn.fn_name.value().as_str())
		};
		let func = match exec_fn.fn_name.parse::<TokenStream>() {
			Ok(f) => f,
			Err(e) => {
				let msg = e.to_string();
				return quote_spanned! {e.span() => {compile_error!{#msg}}}
			}
		};
		let mut tooltip_string = String::new();
		if let Some(t) = &exec_fn.tooltip {
			tooltip_string = t.value().clone();
		}
		let tooltip = tooltip_string.as_str();
		let call_func = if exec_fn.is_method {
			quote! { self.#func(); }
		} else {
			quote! { #func(); }
		};
		quote! {
			egui_field_editor::add_button(#label, #tooltip, read_only, ui, |ui| {
				#call_func();
			});
		}
	});
	quote! {
		#(#recurse)*
	}
}
fn get_code_for_data(data: &Data, struct_name: &Ident) -> TokenStream {
	match *data {
		Data::Struct(ref data) => get_code_for_struct(data),
		Data::Enum(ref an_enum) => get_code_for_enum(struct_name, an_enum),
		Data::Union(_) => unimplemented!("Unions are not supported (would need unsafe code)"),
	}
}

fn get_code_for_struct(data: &DataStruct)  -> TokenStream {
	match data.fields {
		Fields::Named(ref fields) => get_code_for_struct_named_fields(fields),
		Fields::Unnamed(ref fields) => get_code_for_struct_unnamed_fields(fields),
		Fields::Unit => quote! {}
	}
}
/// Generate the code to edit an enum (the content of the ```inspect_with_custom_id``` method)
fn get_code_for_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream {
	let mut variant_texts = Vec::new();
	let mut variant_select_conditions = Vec::new();
	let mut variant_content_edit = Vec::new();
	let mut has_hidden = false;

	for variant in &data_enum.variants {
		let variant_name = &variant.ident;
		let mut label = variant_name.to_string();
		let attrs= match AttributeArgs::from_variant(variant) {
			Ok(_attr) => {
				_attr
			}
			Err(e) => {
				let msg = e.to_string();
				return quote_spanned! {
					e.span() => {
						compile_error!(#msg);
					}
				};
			}
		};
		if attrs.hidden {
			has_hidden = true;
			continue;
		}
		// TODO: if selected variant is transparent , there should be no combobox.
		if let Some(name) = &attrs.name {
			label = name.clone();
		}
		match &variant.fields {
			Fields::Unit => get_code_blocks_for_unit_variant(
				enum_name,
				variant_name,
				label, 
				&mut variant_texts,
				&mut variant_select_conditions,
				&mut variant_content_edit
			),
			Fields::Unnamed(fields) => get_code_blocks_for_unamed_variant(
				enum_name,
				variant_name,
				label,
				fields,
				attrs.read_only,
				&mut variant_texts,
				&mut variant_select_conditions,
				&mut variant_content_edit
			),
			Fields::Named(fields) => get_code_blocks_for_named_variant(
				enum_name,
				variant_name,
				label,
				fields,
				attrs.read_only,
				&mut variant_texts,
				&mut variant_select_conditions,
				&mut variant_content_edit
			)

		}
	}
	
	if has_hidden {
		variant_texts.push(quote!{_ => { "" }});
		variant_content_edit.push(quote! {_ => { ui.response() } });
	}

	quote_spanned! {
		enum_name.span() => {
			let label_ratio = label_ratio.clamp(0.05, 0.95);

			let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
			//TODO: find a way to use it (if only Unit variants) or don't declare it if not needed
			#[allow(unused_variables)]
			let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
			let available_width = ui.available_width();
			let label_width = available_width * label_ratio;
			let field_width = 100.0f32.max(available_width * (1.0-label_ratio) - 10.0);

			let combo_resp = ui.horizontal(|ui| {
				/*let r = ui.add_sized(
					[label_width, 0.0],
					egui::Label::new(label)
						.truncate()
						.show_tooltip_when_elided(true)
						.halign(egui::Align::LEFT),
				);

				if !tooltip.is_empty() {
					r.on_hover_text(tooltip).on_disabled_hover_text(tooltip);
				}*/
				let (rect, _label_res) = ui.allocate_exact_size(
					egui::vec2(label_width, ui.spacing().interact_size.y),
					egui::Sense::hover(),
				);

				let mut child_ui = ui.new_child(egui::UiBuilder::new()
					.max_rect(rect)
					.layout(egui::Layout::left_to_right(egui::Align::Min))

				);

				let label_res = child_ui.add(
					egui::Label::new(label)
						.truncate()
						.show_tooltip_when_elided(true)
						.halign(egui::Align::LEFT));

				if !tooltip.is_empty() {
					if !read_only {
						label_res.on_hover_text(tooltip);
					} else {
						label_res.on_disabled_hover_text(tooltip);
					}
				}
				let mut changed = false;
				ui.add_enabled_ui(!read_only, |ui| {
					let mut resp = egui::ComboBox::from_id_salt(id)
						.width(field_width)
						.selected_text(match self {
							#(#variant_texts)*
						})
						.show_ui(ui, |ui| {
							#(#variant_select_conditions)*
						}).response;
					if changed {
						resp.mark_changed();
					}
					resp
				}).inner
			}).inner;

			let content_resp = match self {
				#(#variant_content_edit)*
			};
			content_resp.union(combo_resp)
		}
	}
}
/// Generate the code to edit an named struct (the content of the ```inspect_with_custom_id``` method)
fn get_code_for_struct_named_fields(fields: &FieldsNamed) -> TokenStream {
	let recurse = fields.named.iter().map(|f| {
		let attrs = match AttributeArgs::from_field(f) {
			Ok(_attrs) => {
				_attrs
			}
			Err(e) => {
				let msg = e.to_string();
				return quote_spanned! { e.span() => {
						compile_error!(#msg);
					}
				};
			}
		};
		if attrs.hidden {
			return quote!();
		}
		let name = &f.ident;

		utils::get_function_call(quote!{&mut self.#name}, f, &attrs, "".into())
	});
	quote_spanned! {
		fields.span() => {
			let mut add_content=|ui:&mut egui::Ui| {
				let mut res: Option<egui::Response> = None;

				macro_rules! combine {
					($r:expr) => {
						if let Some(ref mut total) = res { *total = total.union($r); }
						else { res = Some($r); }
					};
				}
				#(#recurse)*
				res.unwrap_or_else(|| ui.label("Empty Struct"))
			};
			if !label.is_empty() {
				let collapsing_resp = egui::CollapsingHeader::new(label).id_salt(id).show(ui, add_content);
				let mut final_resp = ui.response();
				if let Some(body_res) = collapsing_resp.body_returned {
					final_resp = final_resp.union(body_res);
				}
				final_resp
			} else {
				add_content(ui)
			}
		}
	}
}
/// Generate the code to edit an unnamed struct (the content of the ```inspect_with_custom_id``` method)
fn get_code_for_struct_unnamed_fields(fields: &FieldsUnnamed) -> TokenStream {
	let mut recurse = Vec::new();
	for (i,f) in fields.unnamed.iter().enumerate() {
		let attrs = match AttributeArgs::from_field(f) {
			Ok(_attrs) => {
				_attrs
			}
			Err(e) => {
				let msg = e.to_string();
				return quote_spanned! { e.span() => {
						compile_error!(#msg);
					}
				};
			}
		};
		if attrs.hidden {
			continue;
		}
		let tuple_index = Index::from(i);
		recurse.push(utils::get_function_call(quote!{&mut self.#tuple_index}, f, &attrs, format!("Field {i}")))
	};

	let result = quote_spanned! {
		fields.span() => {
			let mut add_content=|ui:&mut egui::Ui| {
				let mut res: Option<egui::Response> = None;

				macro_rules! combine {
					($r:expr) => {
						if let Some(ref mut total) = res { *total = total.union($r); }
						else { res = Some($r); }
					};
				}
				#(#recurse)*
				res.unwrap_or_else(|| ui.label("Empty UnamedStruct"))
			};
			if !label.is_empty() {
				let collapsing_resp = egui::CollapsingHeader::new(label).id_salt(id).show(ui, add_content);
				if let Some(body_res) = collapsing_resp.body_returned {
					body_res
				} else {
					ui.response()
				}
			} else {
				add_content(ui)
			}
		}
	};
	result
}
/// Fill the ```variant_texts```, ```variant_select_conditions``` and ```variant_content_edit``` code blocks for a unit variant
fn get_code_blocks_for_unit_variant(
		enum_name: &Ident,
		variant_name: &Ident,
		label:String,
		variant_texts:&mut Vec<TokenStream>,
		variant_select_conditions:&mut Vec<TokenStream>,
		variant_content_edit:&mut Vec<TokenStream>) {
	variant_texts.push(quote! {
		#enum_name::#variant_name => #label,
	});

	variant_select_conditions.push(quote! {
		if ui.selectable_value(self, #enum_name::#variant_name, #label).changed() {
			*self = #enum_name::#variant_name;
			changed=true;
		}
	});


	variant_content_edit.push(quote! {
		#enum_name::#variant_name => {
			// nothing to edit
			ui.response()
		}
	});
}
/// Fill the ```variant_texts```, ```variant_select_conditions``` and ```variant_content_edit``` code blocks for a unamed fields variant
#[allow(clippy::too_many_arguments)]
fn get_code_blocks_for_unamed_variant(
	enum_name: &Ident,
	variant_name: &Ident,
	label:String,
	fields : &FieldsUnnamed,
	read_only: bool,
	variant_texts:&mut Vec<TokenStream>,
	variant_select_conditions:&mut Vec<TokenStream>,
	variant_content_edit:&mut Vec<TokenStream>) {

	let default_value = if fields.unnamed.len() == 1 {
		quote! { Default::default() }
	} else {
		let defaults = std::iter::repeat_n(quote! { Default::default() }, fields.unnamed.len());
		quote! {  #(#defaults),*  }
	};
	let bindings_ignore = (0..fields.unnamed.len())
		.map(|_i| Ident::new("_", proc_macro2::Span::call_site()));
	variant_texts.push(quote! {
		#enum_name::#variant_name(#(#bindings_ignore),*) => #label,
	});
	
	variant_select_conditions.push(quote! {
		if ui.selectable_value(self, #enum_name::#variant_name(#default_value), #label).changed() {
			*self = #enum_name::#variant_name(#default_value);
			changed=true;
		}
	});
	let mut fieldnames_list = vec![];
	let bindings = (0..fields.unnamed.len())
		.map(|i| Ident::new(&format!("field{i}"), proc_macro2::Span::call_site()));
	
	let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
		let mut attrs = match AttributeArgs::from_field(f) {
			Ok(_attrs) => {
				_attrs
			}
			Err(e) => {
				let msg = e.to_string();
				return quote_spanned! { e.span() => {
						compile_error!(#msg);
					}
				};
			}
		};
		if attrs.hidden {
			return quote!();
		}
		attrs.read_only = attrs.read_only || read_only;

		let fieldname = format!("field{i}");
		let fieldname = Ident::new(&fieldname, proc_macro2::Span::call_site());
		fieldnames_list.push(quote!{#fieldname});

		utils::get_function_call(quote!{#fieldname}, f, &attrs, format!("Field {i}"))
	});
	let bindings_for_match = bindings.clone();
	variant_content_edit.push(quote! {
		#enum_name::#variant_name(#(#bindings_for_match),* ) => {
			let mut add_content = |ui:&mut egui::Ui| {
				let mut res: Option<egui::Response> = None;

				macro_rules! combine {
					($r:expr) => {
						if let Some(ref mut total) = res { *total = total.union($r); }
						else { res = Some($r); }
					};
				}
				#(#recurse)*
				res.unwrap_or_else(|| ui.label("Empty UnamedStruct"))
			};
			ui.indent(id, add_content).inner
		}
	});
}
/// Fill the ```variant_texts```, ```variant_select_conditions``` and ```variant_content_edit``` code blocks for a named fields variant
#[allow(clippy::too_many_arguments)]
fn get_code_blocks_for_named_variant(
		enum_name: &Ident,
		variant_name: &Ident,
		label:String,
		fields : &FieldsNamed,
		read_only: bool,
		variant_texts:&mut Vec<TokenStream>,
		variant_select_conditions:&mut Vec<TokenStream>,
		variant_content_edit:&mut Vec<TokenStream>) {

	let mut field_bindings = Vec::new();
	let mut inspect_calls = Vec::new();
	
	let bindings_ignore: Vec<TokenStream> = fields.named.iter()
		.map(|f| {
			let name = f.ident.clone();
			quote!{#name : _ }
		}
	).collect();
	variant_texts.push(quote! {
		#enum_name::#variant_name{#(#bindings_ignore),*} => #label,
	});
	
	let defaults = fields.named.iter().map(|field| {
		let name = field.ident.as_ref().unwrap(); // safety: fields is NamedFields
		Some(quote! { #name: Default::default() })
	}).collect::<Vec<_>>();
	let default_value = quote! {  #(#defaults),* };

	variant_select_conditions.push(quote! {
		if ui.selectable_value(self, #enum_name::#variant_name{#default_value}, #label).changed() {
			*self = #enum_name::#variant_name { #default_value };
			changed=true;
		}
	});

	for f in &fields.named {
		let fieldname = f.ident.as_ref().unwrap(); //safety: fields is NamedFields
		let mut hidden = false;
		match AttributeArgs::from_field(f) {
			Ok(mut attrs) => {
				if !attrs.hidden {
					attrs.read_only = attrs.read_only || read_only;
					inspect_calls.push(utils::get_function_call(quote!{#fieldname}, f, &attrs, "".into()));
				}
				hidden = attrs.hidden;
			},
			Err(e) => {
				let msg = e.to_string();
				inspect_calls.push(quote_spanned! { e.span() => {
						compile_error!(#msg);
					}
				});
			}
		}
		if !hidden {
			field_bindings.push(quote!{#fieldname});
		} else {
			field_bindings.push(quote!{#fieldname: _});
		}
	}

	variant_content_edit.push(quote! {
		#enum_name::#variant_name { #( #field_bindings ),* } => {
			ui.indent(id, |ui| {
				let mut res: Option<egui::Response> = None;

				macro_rules! combine {
					($r:expr) => {
						if let Some(ref mut total) = res { *total = total.union($r); }
						else { res = Some($r); }
					};
				}
				#( #inspect_calls )*
				res.unwrap_or_else(|| ui.label("Empty UnamedStruct"))
			}).inner
		}
	});
}