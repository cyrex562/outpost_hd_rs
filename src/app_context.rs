

#[derive(Default,Debug,Clone)]
pub struct AppContext<'a> {
    pub mean_solar_distance: f64,
    pub default_font: &'a mut Font,
}