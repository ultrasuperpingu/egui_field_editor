use std::{cell::RefCell, rc::Rc, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex, RwLock}};

use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_field_editor::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Default)]
#[inspect(
	execute_btn(fn_name="share_refcell"),
	execute_btn(fn_name="share_mutex"),
	execute_btn(fn_name="share_rwlock"),
	execute_btn(fn_name="stop_thread")
)]
struct MyApp {
	refcell_string: Rc<RefCell<String>>,
	mutex_string: Arc<Mutex<String>>,
	rwlock_string: Arc<RwLock<String>>,
	refcell_capture: Option<Rc<RefCell<String>>>,
	#[inspect(hidden)]
	join_handle: Option<std::thread::JoinHandle<()>>,
	#[inspect(hidden)]
	stop_flag: Option<Arc<AtomicBool>>,
	#[inspect(read_only)]
	thread_running: bool,
}

impl MyApp {
	fn share_refcell(&mut self) {
		self.refcell_capture = Some(self.refcell_string.clone());
	}
	fn share_mutex(&mut self) {
		self.stop_thread();
		let stop_flag = Arc::new(AtomicBool::new(false));
		self.stop_flag = Some(stop_flag.clone());

		let string_arc = Arc::clone(&self.mutex_string);
		self.join_handle = Some(std::thread::spawn(move || {
			while !stop_flag.load(Ordering::SeqCst) {
				{
					let mut s = string_arc.lock().unwrap();
					s.push_str(".");
				}
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
		}));
		self.thread_running = true;
	}
	fn share_rwlock(&mut self) {
		self.stop_thread();

		let stop_flag = Arc::new(AtomicBool::new(false));
		self.stop_flag = Some(stop_flag.clone());

		let string_arc = Arc::clone(&self.rwlock_string);

		self.join_handle = Some(std::thread::spawn(move || {
			while !stop_flag.load(Ordering::SeqCst) {
				{
					let mut s = string_arc.write().unwrap();
					s.push_str(".");
				}
				std::thread::sleep(std::time::Duration::from_millis(1000));
			}
		}));
		self.thread_running = true;
	}
	fn stop_thread(&mut self) {
		if let Some(h) = self.join_handle.take() {
			if let Some(flag) = &self.stop_flag {
				flag.store(true, Ordering::SeqCst);
			}
			h.join().unwrap();
		}
		self.thread_running = false;
	}
}
impl eframe::App for MyApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		let code = include_str!("shared_data.rs");
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
