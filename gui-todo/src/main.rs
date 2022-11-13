#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use ::gui_todo::gui_todo::Todo;
use eframe::egui;
use gui_todo::gui_todo::Id;

const APP_NAME: &str = "GUI TODO";

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(PartialEq)]
enum Filter {
    All,
    Completed,
    Uncompleted,
}

struct MyApp {
    todos: Vec<Todo>,
    new_todo: String,
    id: Id,
    filter: Filter,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            todos: Vec::new(),
            new_todo: String::new(),
            id: Id::new(),
            filter: Filter::All,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(APP_NAME);
            ui.horizontal(|ui| {
                ui.label("Todo: ");
                ui.text_edit_singleline(&mut self.new_todo);
                if ui.button("Add").clicked() {
                    self.todos
                        .push(Todo::new(self.id.next(), self.new_todo.clone()));
                    self.new_todo.clear();
                }
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.filter, Filter::All, "All");
                ui.selectable_value(&mut self.filter, Filter::Completed, "Completed");
                ui.selectable_value(&mut self.filter, Filter::Uncompleted, "Uncompleted");
            });

            ui.vertical(|ui| {
                for todo in &mut self.todos {
                    if self.filter == Filter::Completed && !todo.is_completed {
                        continue;
                    }
                    if self.filter == Filter::Uncompleted && todo.is_completed {
                        continue;
                    }
                    todo.view(ui);
                }
            })
        });
    }
}
