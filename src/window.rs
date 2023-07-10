#[derive(Default,Debug,Clone)]
pub struct Window<'a>
{
    pub ui_container: UIContainer,
    pub title_font: &'a mut Font,
    pub title_bar_left: &'a mut Image,
    pub title_bar_center: &'a mut Image,
    pub title_bar_right: &'a mut Image,
    pub body: RectangleSkin,
    pub title_changed: TitleChangeSignal,
    pub title: String,
    pub mouse_drag: bool,
    pub anchored: bool,
}