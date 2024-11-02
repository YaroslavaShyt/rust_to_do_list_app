use druid::{widget::Label, Widget};
use crate::data::AppState;

pub fn build_ui() -> impl Widget<AppState> {
    Label::new("Hello")
}
