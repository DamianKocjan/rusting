pub mod gui_todo {
    use eframe::egui;

    pub struct Id {
        pub id: usize,
    }

    impl Id {
        pub fn new() -> Self {
            Self { id: 0 }
        }

        pub fn next(&mut self) -> usize {
            self.id = self.id + 1;
            self.id
        }
    }

    pub struct Todo {
        pub id: usize,
        pub title: String,
        pub is_completed: bool,
    }

    impl Todo {
        pub fn new(id: usize, title: String) -> Self {
            Self {
                id,
                title,
                is_completed: false,
            }
        }

        pub fn view(&mut self, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                ui.label(&self.title);
                ui.checkbox(&mut self.is_completed, "Completed");
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let mut id = gui_todo::Id::new();
        assert_eq!(id.next(), 1);
        assert_eq!(id.next(), 2);
        assert_eq!(id.next(), 3);
    }

    #[test]
    fn test_todo() {
        let todo = gui_todo::Todo::new(1, "Todo 1".to_owned());
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Todo 1");
        assert_eq!(todo.is_completed, false);
    }
}
