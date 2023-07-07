

#[derive(Default,Debug,Clone)]
pub struct Frame<'a>
{
    image: &'a Image,
    bounds: Rectangle<i32>,
    anchor_offset: Vec<i32>,
    frame_delay: u32
}

impl Frame
{
    pub fn is_stop_frame(&mut self) -> bool
    {
        self.frame_delay == 0 || self.frame_delay == u32::MAX
    }


}