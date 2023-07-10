#[derive(Default,Debug,Clone)]
pub struct UiContainer {
    pub control: Control,
    pub controls: Vec<Control>
}

impl UiContainer {
    pub fn new(controls: &mut Vec<Control>) {

    }
}