mod app;

fn main() {
    unsafe {
        let mut app = app::ToDoListApp::new();
        app.run();
    }
}
