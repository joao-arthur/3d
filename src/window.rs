use winit::{
    dpi::{PhysicalSize, Size},
    error::OsError,
    event_loop::ActiveEventLoop,
    window::Window,
};

pub fn get_winit_window(event_loop: &ActiveEventLoop) -> Result<Window, OsError> {
    let attributes = Window::default_attributes()
        .with_inner_size(Size::Physical(PhysicalSize::new(100, 100)))
        .with_visible(true)
        .with_active(true)
        .with_title("Title of the window");
    event_loop.create_window(attributes)
}
