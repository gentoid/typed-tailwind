use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[derive(ScreenSize)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Padding(pub Side, pub Space);

impl From<&Padding> for String {
    fn from(padding: &Padding) -> String {
        match padding {
            Padding(side, space) => format!("p{}{}", String::from(side), String::from(space)),
        }
    }
}

#[derive(ScreenSize)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Margin(pub Side, pub Space);

impl From<&Margin> for String {
    fn from(margin: &Margin) -> String {
        match margin {
            Margin(side, space) => format!("m{}{}", String::from(side), String::from(space)),
        }
    }
}

pub enum Side {
    All,
    X,
    Y,
    Top,
    Bottom,
    Right,
    Left,
}

impl From<&Side> for String {
    fn from(kind: &Side) -> Self {
        use Side::*;

        match kind {
            All => "",
            X => "-x",
            Y => "-y",
            Top => "-t",
            Bottom => "-b",
            Right => "-r",
            Left => "-l",
        }
        .into()
    }
}

pub enum Space {
    _0,
    _0_5,
    _1,
    _1_5,
    _2,
    _2_5,
    _3,
    _3_5,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _14,
    _16,
    _20,
    _24,
    _28,
    _32,
    _36,
    _40,
    _44,
    _48,
    _52,
    _56,
    _60,
    _64,
    _72,
    _80,
    _96,
    Px,
}

impl From<&Space> for String {
    fn from(size: &Space) -> Self {
        use Space::*;

        match size {
            _0 => "-0",
            _0_5 => "-0.5",
            _1 => "-1",
            _1_5 => "-1.5",
            _2 => "-2",
            _2_5 => "-2.5",
            _3 => "-3",
            _3_5 => "-3.5",
            _4 => "-4",
            _5 => "-5",
            _6 => "-6",
            _7 => "-7",
            _8 => "-8",
            _9 => "-9",
            _10 => "-10",
            _11 => "-11",
            _12 => "-12",
            _14 => "-14",
            _16 => "-16",
            _20 => "-20",
            _24 => "-24",
            _28 => "-28",
            _32 => "-32",
            _36 => "-36",
            _40 => "-40",
            _44 => "-44",
            _48 => "-48",
            _52 => "-52",
            _56 => "-56",
            _60 => "-60",
            _64 => "-64",
            _72 => "-72",
            _80 => "-80",
            _96 => "-96",
            Px => "-px",
        }
        .into()
    }
}
