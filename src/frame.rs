

#[derive(Default,Debug,Clone)]
pub struct Frame<'a>
{
    pub image: &'a Image,
    pub bounds: Rectangle<i32>,
    pub anchor_offset: Vec<i32>,
    pub frame_delay: u32
}

impl Frame
{
    pub fn is_stop_frame(&mut self) -> bool
    {
        self.frame_delay == 0 || self.frame_delay == u32::MAX
    }


}