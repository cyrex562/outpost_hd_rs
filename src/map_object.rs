

// type DieSignal = Signal<MapObject>;

#[derive(Default,Debug,Clone)]
pub struct MapObject
{
    pub name: String,
    pub sprite: Sprite,
    pub die_signal: Signal<MapObject>,
    pub is_dead: bool
}

impl MapObject
{
    pub fn new(name: &str, sprite_path: &str, initial_action: &str) -> Self {
        Self {
            name,
            sprite: Sprite::new(sprite_path, initial_action),
            ..Default::default()
        }
    }

    pub fn die(&mut self)
    {
        self.is_dead = true;
        self.die_signal(self);
    }

    pub fn on_die(&mut self) -> Signal<MapObject>
    {
        self.die_signal
    }
}