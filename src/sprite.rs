use sdl2::hint::Hint::Default;
use crate::animation_set::AnimationSet;
use crate::color::Color;
use crate::frame::Frame;
use crate::point::Point;
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

    pub fn actions(&mut self) -> Vec<String> {
        self.animation_set.action_names()
    }

    pub fn play(&mut self, action: &str)
    {
        self.current_action = self.animation_set.frames(action);
        self.current_frame = 0;
        self.timer.reset();
        self.resume();
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn set_frame(&mut self, frame_index: usize) {
        self.current_frame = frame_index % self.current_action.len();
    }

    pub fn update(&mut self) {
        self.timer.adjust_start_tick((advance_time_by_delta(self.timer.elapsed_ticks())));
    }

    pub fn draw(&mut self, position: Point<f64>) {
        let frame = self.current_action[self.current_frame].clone();
        let draw_position = position - frame.anchor_offset.into();
        let frame_bounds = frame.bounds.into();
        // Utility<Renderer>::get().drawSubImageRotated(frame.image, drawPosition, frameBounds, mRotationAngleDegrees, mTintColor);
        // TODO
    }

    pub fn advance_time_by_delta(&mut self, time_delta: u32) -> u32
    {
        let mut accumulator = 0u32;
        if self.paused {
            return accumulator;
        }

        loop {
            let frame = &mut self.current_action[self.current_frame];
            if frame.is_stop_frame() {
                self.animation_complete_signal = true;
            }
            if time_delta - accumulator < frame.frame_delay {
                return accumulator;
            }

            accumulator += frame.frame_delay;
            self.current_frame += 1;
            if self.current_frame >= self.current_action.len() {
                self.current_frame = 0;
                self.animation_complete_signal = true;
            }
        }
    }
}