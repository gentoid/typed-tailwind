pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl From<TextAlign> for String {
    fn from(size: TextAlign) -> Self {
        use TextAlign::*;

        match size {
            Left => "-left",
            Center => "-center",
            Right => "-right",
            Justify => "-justify",
        }
        .into()
    }
}

pub enum TextSize {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    _2xl,
    _3xl,
    _4xl,
    _5xl,
    _6xl,
    _7xl,
    _8xl,
    _9xl,
}

impl From<TextSize> for String {
    fn from(size: TextSize) -> Self {
        use TextSize::*;

        match size {
            Xs => "-xs",
            Sm => "-sm",
            Base => "-base",
            Lg => "-lg",
            Xl => "-xl",
            _2xl => "-2xl",
            _3xl => "-3xl",
            _4xl => "-4xl",
            _5xl => "-5xl",
            _6xl => "-6xl",
            _7xl => "-7xl",
            _8xl => "-8xl",
            _9xl => "-9xl",
        }
        .into()
    }
}

pub enum FontWeight {
    Thin,
    Extralight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    Extrabold,
    Black,
}

impl From<FontWeight> for String {
    fn from(weight: FontWeight) -> Self {
        use FontWeight::*;

        match weight {
            Thin => "-thin",
            Extralight => "-extralight",
            Light => "-light",
            Normal => "-normal",
            Medium => "-medium",
            Semibold => "-semibold",
            Bold => "-bold",
            Extrabold => "-extrabold",
            Black => "-black",
        }
        .into()
    }
}

pub enum Leading {
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    None,
    Tight,
    Snug,
    Normal,
    Relaxed,
    Loose,
}

impl From<Leading> for String {
    fn from(leading: Leading) -> Self {
        use Leading::*;

        match leading {
            _3 => "-3",
            _4 => "-4",
            _5 => "-5",
            _6 => "-6",
            _7 => "-7",
            _8 => "-8",
            _9 => "-9",
            _10 => "-10",
            Leading::None => "-none",
            Tight => "-tight",
            Snug => "-snug",
            Normal => "-normal",
            Relaxed => "-relaxed",
            Loose => "-loose",
        }
        .into()
    }
}
