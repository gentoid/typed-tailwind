use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Border(pub BorderSide, pub BorderWidth);

impl From<&Border> for String {
    fn from(border: &Border) -> String {
        match border {
            Border(side, width) => format!("border{}{}", String::from(side), String::from(width)),
        }
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Rounded(pub RadiusSide, pub RadiusSize);

impl From<&Rounded> for String {
    fn from(rounded: &Rounded) -> String {
        match rounded {
            Rounded(side, size) => format!("rounded{}{}", String::from(side), String::from(size)),
        }
    }
}

#[derive(FocusState, HoverState)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct BorderColor(pub Color);

impl From<&BorderColor> for String {
    fn from(border_color: &BorderColor) -> String {
        match border_color {
            BorderColor(color) => format!("border{}", String::from(color)),
        }
    }
}

#[derive(FocusState)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct RingWidth(pub RingWidthVal);

impl From<&RingWidth> for String {
    fn from(ring_width: &RingWidth) -> String {
        match ring_width {
            RingWidth(width) => format!("ring{}", String::from(width)),
        }
    }
}

pub enum RingWidthVal {
    _0,
    _1,
    _2,
    _4,
    _8,
    Default,
    Inset,
}

impl From<&RingWidthVal> for String {
    fn from(ring_width: &RingWidthVal) -> Self {
        use RingWidthVal::*;

        match ring_width {
            _0 => "-0",
            _1 => "-1",
            _2 => "-2",
            _4 => "-4",
            _8 => "-8",
            Default => "-default",
            Inset => "-inset",
        }
        .into()
    }
}

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

impl From<&RadiusSize> for String {
    fn from(size: &RadiusSize) -> Self {
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

impl From<&RadiusSide> for String {
    fn from(side: &RadiusSide) -> Self {
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

impl From<&BorderSide> for String {
    fn from(side: &BorderSide) -> Self {
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

impl From<&BorderWidth> for String {
    fn from(size: &BorderWidth) -> Self {
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
