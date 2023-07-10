
pub struct Renderer
{
    pub window: Window,

}

impl Renderer
{
    pub fn new(app_title: &str) -> Self
    {
        Self {
            window: Window::new(app_title)
        }
    }
}