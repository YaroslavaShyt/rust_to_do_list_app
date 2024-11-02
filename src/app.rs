use fltk::{
    app,
    group::{Flex, Scroll},
    input::Input,
    prelude::*,
    window::Window,
};

mod to_do_list;
use crate::app::to_do_list::Visualizer;
mod crud;
use crate::app::crud::CRUD;

#[derive(Copy, Clone)]
pub enum Action {
    MarkAsDone(&'static str),
    Reset(&'static str),
    Remove(&'static str, &'static str),
    Add(),
}

pub struct ToDoListApp {
    app: app::App,
    window: Window,
    to_do_list: Vec<String>,
    receiver: app::Receiver<Action>,
    sender: app::Sender<Action>,
}

impl ToDoListApp {
    pub fn new() -> Self {
        let to_do_list = ToDoListApp::read_todo_from_db().unwrap();
        let app = app::App::default();
        let (sender, receiver) = app::channel();

        let window = Window::default()
            .with_size(800, 400)
            .with_label("To-do-list App");

        Self {
            app: app,
            window: window,
            to_do_list: to_do_list,
            sender: sender,
            receiver: receiver,
        }
    }

    pub unsafe fn run(&mut self) {
        self.view();
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    Action::MarkAsDone(task_name) => {
                        self.change_task_state(task_name, "Done");
                    }
                    Action::Reset(task_name) => {
                        self.change_task_state(task_name, "Not Done");
                    }
                    Action::Remove(task_name, task_state) => {
                        self.remove_task(task_name, task_state);
                    }
                    Action::Add() => {
                        let scrool = Scroll::from_widget(self.window.child(0).unwrap());
                        let flex_add = Flex::from_widget(scrool.child(0).unwrap());
                        let add_input = Input::from_widget(flex_add.child(0).unwrap());

                        self.add_task(&add_input.value(), "Not Done");
                    }
                };
                self.view();
            }
        }
    }
}
