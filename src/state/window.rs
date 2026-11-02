use winit::{error::OsError, event_loop::ActiveEventLoop, window::Window};

pub fn get_winit_window(event_loop: &ActiveEventLoop) -> Result<Window, OsError> {
    event_loop.create_window(
        Window::default_attributes()
            .with_visible(true)
            .with_active(true)
            .with_title("Title of the window"),
    )
}
