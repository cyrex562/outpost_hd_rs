use crate::animation_set::AnimationSet;
use crate::color::Color;
use crate::frame::Frame;

type AnimationCompleteSignal = Signal<()>;

#[derive(Default,Debug,Clone)]
pub struct Sprite<'a>
{
    pub animation_set: &'a AnimationSet<'a>,
    pub current_action: Vec<Frame<'a>>,
    pub current_frame: usize,
    pub paused: bool,
    pub timer: Timer,
    pub animation_complete_signal: AnimationCompleteSignal,
    pub tint_color: Color,
    pub rotation_angle_degrees: f64
}

impl Sprite
{

}