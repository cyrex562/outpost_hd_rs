


#[derive(Default,Debug,Clone,PartialOrd)]
pub struct Color
{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color
{
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha
        }
    }
    pub fn alpha_fade(&mut self, new_alpha: u8) -> Color
    {
        Color{
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: new_alpha
        }
    }
}

pub const Black: Color = Color{
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};

pub const Blue: Color = Color{
    red: 0,
    green: 0,
    blue: 255,
    alpha: 255,
};

pub const Green: Color = Color{
    red: 0,
    green: 255,
    blue: 0,
    alpha: 255,
};

pub const Cyan: Color = Color{
    red: 0,
    green: 255,
    blue: 255,
    alpha: 255,
};

pub const DarkGreen: Color = Color{
    red: 0,
    green: 128,
    blue: 0,
    alpha: 255,
};

pub const DarkGray: Color = Color{
    red: 64,
    green: 64,
    blue: 64,
    alpha: 255,
};

pub const Gray: Color = Color{
    red: 128,
    green: 128,
    blue: 128,
    alpha: 255,
};

pub const LightGray: Color = Color{
    red: 192,
    green: 192,
    blue: 192,
    alpha: 255,
};

pub const CoolGray: Color = Color{
    red: 128,
    green: 128,
    blue: 143,
    alpha: 255,
};

pub const CoolLightGray: Color = Color{
    red: 192,
    green: 192,
    blue: 207,
    alpha: 255,
};

pub const CoolDarkGray: Color = Color{
    red: 64,
    green: 64,
    blue: 79,
    alpha: 255,
};

pub const WarmGray: Color = Color{
    red: 143,
    green: 143,
    blue: 128,
    alpha: 255,
};

pub const WarmLightGray: Color = Color{
    red: 207,
    green: 207,
    blue: 192,
    alpha: 255,
};

pub const WarmDarkGray: Color = Color{
    red: 79,
    green: 79,
    blue: 64,
    alpha: 255,
};

pub const Magenta: Color = Color{
    red: 255,
    green: 0,
    blue: 255,
    alpha: 255,
};

pub const Navy: Color = Color{
    red: 35,
    green: 60,
    blue: 85,
    alpha: 255,
};

pub const Orange: Color = Color{
    red: 255,
    green: 128,
    blue: 0,
    alpha: 255,
};

pub const Red: Color = Color{
    red: 255,
    green: 0,
    blue: 0,
    alpha: 255,
};

pub const Silver: Color = Color{
    red: 192,
    green: 192,
    blue: 192,
    alpha: 255,
};

pub const White: Color = Color{
    red: 255,
    green: 255,
    blue: 255,
    alpha: 255,
};

pub const Yellow: Color = Color{
    red: 255,
    green: 255,
    blue: 0,
    alpha: 255,
};

pub const Normal: Color = Color{
    red: 255,
    green: 255,
    blue: 255,
    alpha: 255,
};

pub const NormalZ: Color = Color{
    red: 128,
    green: 128,
    blue: 255,
    alpha: 255,
};

pub const NoAlpha: Color = Color{
    red: 0,
    green: 0,
    blue: 0,
    alpha: 0,
};

