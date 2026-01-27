use crate::application::Application;

mod application;
mod utils;

pub fn main() -> iced::Result {
    iced::application(Application::new, Application::update, Application::view)
        .subscription(Application::subscription)
        .title("Shutdown Timer")
        .window_size((application::WINDOW_WIDTH, application::WINDOW_HEIGHT))
        .resizable(false)
        .run()
}