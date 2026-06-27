## v0.6.0
 - Port to egui 0.35
## v0.5.2
 - Remove useless and intrusive need to implement EguiInspect for generic types.
 - Fix splitter inspector id
 - Last Egui 0.34 version
## v0.5.1
 - Fix panic caused by merging responses from different layers (layer_id mismatch)
 - Remove duplicated macro expansion in generated code
 - Replace removed macro logic with a #[doc(hidden)] helper macro in the egui_field_editor crate
## v0.5.0
 - Port to egui 0.34
 - Fix Vec<enum>
 - Align labels left
 - Add a splitter to modify label_ratio in Inspector
 - Add Stroke
 - Docs update
## v0.4.0
 - Add hashmap, smallvec and arrayvec support
## v0.3.0
 - Add label_ratio parameters to inspect functions
## v0.2.2
 - Add file picker
## v0.2.1
 - Docs update
## v0.2.0
 - First version of egui_field_editor
 - Full refactor
 - Add enum support
 - Add execution_btn
 - Add nalbegra_glm support
 - Add datepicker support
 - Implement more basic types
## v0.1.2
- Add field attribute `no_edit`, `skip`, `custom_func`, `custom_func_mut`
- Small refactoring