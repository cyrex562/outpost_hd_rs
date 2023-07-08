

// type DieSignal = Signal<MapObject>;

use crate::sprite::Sprite;

#[derive(Default,Debug,Clone)]
pub struct MapObject<'a>
{
    pub name: String,
    pub sprite: Sprite<'a>,
    pub die_signal: Signal<MapObject<'a>>,
    pub is_dead: bool
}

impl MapObject
{
    pub fn new(name: &str, sprite_path: &str, initial_action: &str) -> Self {
        Self {
            name: name.to_string(),
            sprite: Sprite::new(None, None,sprite_path, initial_action),
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