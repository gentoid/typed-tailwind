pub enum RadiusSize {
    None,
    Sm,
    Default,
    Md,
    Lg,
    Xl,
    _2xl,
    _3xl,
    Full,
}

impl From<RadiusSize> for String {
    fn from(size: RadiusSize) -> Self {
        use RadiusSize::*;

        match size {
            RadiusSize::None => "-none",
            Sm => "-sm",
            Default => "",
            Md => "-md",
            Lg => "-lg",
            Xl => "-xl",
            _2xl => "-2xl",
            _3xl => "-3xl",
            Full => "-full",
        }
        .into()
    }
}

pub enum RadiusSide {
    All,
    Top,
    Right,
    Bottom,
    Left,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl From<RadiusSide> for String {
    fn from(side: RadiusSide) -> Self {
        use RadiusSide::*;

        match side {
            All => "",
            Top => "-t",
            Right => "-r",
            Bottom => "-b",
            Left => "-l",
            TopRight => "-tr",
            TopLeft => "-tl",
            BottomRight => "-br",
            BottomLeft => "-bl",
        }
        .into()
    }
}

pub enum BorderSide {
    All,
    Top,
    Right,
    Bottom,
    Left,
}

impl From<BorderSide> for String {
    fn from(side: BorderSide) -> Self {
        use BorderSide::*;

        match side {
            All => "",
            Top => "-t",
            Right => "-r",
            Bottom => "-b",
            Left => "-l",
        }
        .into()
    }
}

pub enum BorderWidth {
    Default,
    _0,
    _2,
    _4,
    _8,
}

impl From<BorderWidth> for String {
    fn from(size: BorderWidth) -> Self {
        use BorderWidth::*;

        match size {
            Default => "",
            _0 => "-0",
            _2 => "-2",
            _4 => "-4",
            _8 => "-8",
        }
        .into()
    }
}
