
use chrono::NaiveDate;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;



#[derive(EguiInspect)]
struct MyApp{
	pub naive_date:NaiveDate,
	#[inspect(date(calendar_week=false, highlight_weekends=false, start_end_years(min=2015, max=2028), combo_boxes=false))]
	pub naive_date2:NaiveDate
}
impl Default for MyApp {
	fn default() -> Self {
		Self { naive_date: chrono::Local::now().date_naive(), naive_date2: chrono::Local::now().date_naive() }
	}
}
impl eframe::App for MyApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		let code = include_str!("datepicker.rs");
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
