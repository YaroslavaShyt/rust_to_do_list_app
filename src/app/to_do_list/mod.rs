use crate::{app::Action, app::ToDoListApp};
use fltk::{button::Button, frame::Frame, group::Flex, input::Input, prelude::*, *};
use group::Scroll;

static mut TASK_VEC: Vec<String> = vec![];

pub trait Visualizer {
    unsafe fn view(&mut self);
    fn clear_window(&mut self);
}

impl Visualizer for ToDoListApp {
    unsafe fn view(&mut self) {
        self.clear_window();

        TASK_VEC = self.to_do_list.clone();

        let mut offset = 40;

        self.window.begin();

        let scroll_bar = Scroll::default().with_size(800, 600);

        let flex_add = Flex::default().with_size(780, 30).with_pos(0, 0).row();

        let mut add_input = Input::default();
        add_input.take_focus().unwrap();
        add_input.set_trigger(enums::CallbackTrigger::EnterKey);
        add_input.emit(self.sender, Action::Add());

        let mut add_btn = Button::default().with_label("Додати");
        add_btn.emit(self.sender, Action::Add());

        flex_add.end();

        for task in TASK_VEC.iter() {
            let vec: Vec<&str> = task.split(" -> ").collect();
            let task = vec[0];
            let current_state = vec[1];

            let flex = Flex::default().with_size(780, 30).with_pos(0, offset).row();

            Frame::default().with_label(task);
            Frame::default().with_label(current_state);

            let mut reset_btn = Button::default().with_label("Скинути");
            let mut done_btn = Button::default().with_label("Виконано");
            let mut remove_btn = Button::default().with_label("Видалити");

            done_btn.emit(self.sender, Action::MarkAsDone(task));
            reset_btn.emit(self.sender, Action::Reset(task));
            remove_btn.emit(self.sender, Action::Remove(task, current_state));

            flex.end();

            offset += 50;
        }

        scroll_bar.end();

        self.window.end();
        self.window.show();
    }

    fn clear_window(&mut self) {
        self.window.clear();
        self.window.redraw();
    }
}
