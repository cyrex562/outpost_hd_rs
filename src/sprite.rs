use sdl2::hint::Hint::Default;
use crate::animation_set::AnimationSet;
use crate::color::Color;
use crate::frame::Frame;
use crate::timer::Timer;

type AnimationCompleteSignal = Signal<()>;
type AnimationCache<'a> = ResourceCache<AnimationSet<'a>, String>;


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

impl Sprite {
    pub fn new(animation_cache: Option<&AnimationCache>, animation_set: Option<&AnimationSet>, file_path: &str, initial_action: &str) -> Self {
        let mut anim_set: AnimationSet;
        if animation_cache.is_some() && animation_set.is_none() {
            anim_set = animation_cache.unwrap().load();
        } else if animation_cache.is_none() && animation_set.is_some() {
            anim_set = animation_set.unwrap().clone();
        } else {
            panic!("animation cache and animation set cannot both be non-null")
        }

        let mut out = Self {
            animation_set: &anim_set,
            current_action: anim_set.frames(initial_action),
            ..Default::default()
        };
        out
    }

    pub fn size(&mut self) -> Vec<i32> {
        // return (*mCurrentAction)[mCurrentFrame].bounds.size;
        // self.current_action[self.current_frame]
        todo!()
    }

    pub fn origin(&mut self, point: Point<i32>) -> Point<i32>
    {
        point - self.current_action[self.current_frame].anchor_offset.clone()
    }
}