use crate::rectangle::Rectangle;
use crate::vector_2d::Vector2D;

pub struct GlyphMetrics
{
    pub uv_rect: Rectangle<f64>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub advance: i32,
}

pub struct FontInfo
{
    pub texture_id: u32,
    pub point_size: u32,
    pub height: i32,
    pub ascent: i32,
    pub glyph_size: Vector2D<i32>,
    pub metrics: Vec<GlyphMetrics>
}


pub struct Font
{
    pub font_info: FontInfo,
}

impl Font
{

}