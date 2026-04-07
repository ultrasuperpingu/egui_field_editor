
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;



#[derive(EguiInspect)]
struct MyApp{
	pub jiff_date:jiff::civil::Date,
	#[inspect(date(calendar_week=false, highlight_weekends=false, start_end_years(min=2015, max=2028), combo_boxes=false))]
	pub jiff_date2:jiff::civil::Date,
	#[cfg(feature = "chrono")]
	pub naive_date:chrono::NaiveDate,
	#[inspect(date(calendar_week=false, highlight_weekends=false, start_end_years(min=2015, max=2028), combo_boxes=false))]
	#[cfg(feature = "chrono")]
	pub naive_date2:chrono::NaiveDate,
	#[cfg(feature = "time")]
	pub time_date:time::Date,
	#[inspect(date(calendar_week=false, highlight_weekends=false, start_end_years(min=2015, max=2028), combo_boxes=false))]
	#[cfg(feature = "time")]
	pub time_date2:time::Date
}
impl Default for MyApp {
	fn default() -> Self {
		Self {
			jiff_date: jiff::civil::Date::constant(2000, 12, 31),
			jiff_date2: jiff::civil::Date::constant(2000, 12, 31),
			#[cfg(feature = "chrono")]
			naive_date: chrono::Local::now().date_naive(),
			#[cfg(feature = "chrono")]
			naive_date2: chrono::Local::now().date_naive(),
			#[cfg(feature = "time")]
			time_date: time::Date::from_calendar_date(2000, time::Month::December, 31).unwrap(),
			#[cfg(feature = "time")]
			time_date2: time::Date::from_calendar_date(2000, time::Month::December, 31).unwrap()
		}
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
