use sdl2::sys::SDL_GetTicks;

#[derive(Default,Debug,Clone)]
pub struct Timer
{
    pub start_tick: u32,
}

impl Timer{
    pub fn tick() -> u32
    {
        let result = unsafe { SDL_GetTicks() };
        result
    }

    pub fn new(start_tick: Option<u32>) -> Self
    {
        let ticks = start_tick.unwrap_or(Self::tick())
        Self {
            start_tick: ticks,
        }
    }

    pub fn elapsed_ticks(&mut self) -> u32 {
        Self::tick() - self.start_tick
    }

    pub fn delta(&mut self) -> u32 {
        let elapased = self.elapsed_ticks();
        self.adjust_start_tick(elapsed);
        elapsed
    }

    pub fn adjust_start_tick(&mut self, ticks_forward: u32) {
        self.start_tick += ticks_forward;
    }

    pub fn reset(&mut self) {
        self.start_tick = Self::tick();
    }
}