use crate::app_context::AppContext;
use crate::point::Point;
use crate::rectangle::Rectangle;
use crate::signal::Signal;
use crate::vector_2d::Vector2D;

type ResizeSignal = Signal<Control>;
type OnMoveSignal = Signal<Vector2D<i32>>;
type ControlImageCache = ResourceCache<Image, String>;

#[derive(Default,Debug,Clone)]
pub struct Control
{
    pub on_move_signal: OnMoveSignal,
    pub on_resize_signal: ResizeSignal,
    pub rect: Rectangle<i32>,
    pub visiable: bool,
    pub enabled: bool,
    pub has_focus: bool,
    pub highlight: bool,
}

impl Control {
    pub fn on_move(&mut self, displacement: Vector2D<i32>) {
        self.on_move_signal(displacement)
    }

    pub fn on_resize(&mut self) {
        self.on_resize_signal(self)
    }

    pub fn on_visibility_change(&mut self, visible: bool) {
        todo!()
    }

    pub fn on_enable_change(&mut self) {
        todo!()
    }

    pub fn on_focus_change(&mut self) {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn size(&mut self) -> Vector2D<i32>
    {
        self.rect.size.clone()
    }

    pub fn hide(&mut self) {
        self.visiable(false);
    }

    pub fn show(&mut self) {
        self.visible(true);
    }

    pub fn position(&mut self) -> Point<i32> {
        self.rect.position.clone();
    }

    pub fn set_default_font(&mut self, ctx: &mut AppContext, font: &mut Font) {
        self.ctx.default_font = font;
    }
}