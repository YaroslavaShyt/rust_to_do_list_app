use std::{fs, io};
use crate::app::ToDoListApp;



pub trait CRUD {
    fn read_todo_from_db () -> Result<Vec<String>, io::Error> {
        let file = fs::OpenOptions::new()
                                     .write(true)
                                     .create(true)
                                     .read(true)
                                     .open("db.json")
                                     .unwrap();
         match serde_json::from_reader(file) {
            Ok(todo_list) => Ok( todo_list ),
            Err(e) if e.is_eof() => Ok( vec![] ),
            Err(e) => panic!("Трапила сі помилка :(: {}", e)
         }
   }

    fn save (&self);

    fn change_task_state (&mut self, task_name: &str, task_state: &str);

    fn add_task (&mut self, task_name: &str, task_state: &str);

    fn remove_task (&mut self, task_name: &str, task_state: &str);
}

impl CRUD for ToDoListApp {
    
    fn save (&self) {
        std::fs::write(
            "db.json", 
            serde_json::to_string_pretty(&self.to_do_list).unwrap()
        ).unwrap();
    }

    fn change_task_state (&mut self, task_name: &str, task_state: &str) {
        let prev_task_state = if task_state == "Done" { "Not Done" } else { "Done" };
        let prev_task = format!("{} -> {}", task_name, prev_task_state);
        
        if let Some(task) = self.to_do_list.iter_mut().find(|el| **el == prev_task) {
            *task = format!("{} -> {}", task_name, task_state);
            self.save();
        } 
    }

    fn add_task (&mut self, task_name: &str, task_state: &str) {
        let task = format!("{} -> {}", task_name, task_state);

        if let None = self.to_do_list.iter().find(|el| **el == task) { 
            self.to_do_list.push(task);
            self.save();
        } 
    }

    fn remove_task (&mut self, task_name: &str, task_state: &str) {
        let task = format!("{} -> {}", task_name, task_state);

        if let Some(index) = self.to_do_list.iter().position(|el| *el == task) {
            self.to_do_list.remove(index);
            self.save();
        }
    }
}