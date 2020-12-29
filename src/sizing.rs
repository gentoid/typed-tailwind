use crate::core::*;
use typed_tailwind_derive::*;

#[derive(ScreenSize)]
pub struct Width(Size);

impl From<&Width> for String {
    fn from(width: &Width) -> Self {
        match width {
            Width(size) => format!("width{}", String::from(size)),
        }
    }
}

pub enum Size {
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
    Auto,
    Px,
    _1of2,
    _1of3,
    _2of3,
    _1of4,
    _2of4,
    _3of4,
    _1of5,
    _2of5,
    _3of5,
    _4of5,
    _1of6,
    _2of6,
    _3of6,
    _4of6,
    _5of6,
    _1of12,
    _2of12,
    _3of12,
    _4of12,
    _5of12,
    _6of12,
    _7of12,
    _8of12,
    _9of12,
    _10of12,
    _11of12,
    Full,
    Screen,
    Min,
    Max,
}

impl From<&Size> for String {
    fn from(size: &Size) -> Self {
        use Size::*;

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
            Auto => "-auto",
            Px => "-px",
            _1of2 => "-1/2",
            _1of3 => "-1/3",
            _2of3 => "-2/3",
            _1of4 => "-1/4",
            _2of4 => "-2/4",
            _3of4 => "-3/4",
            _1of5 => "-1/5",
            _2of5 => "-2/5",
            _3of5 => "-3/5",
            _4of5 => "-4/5",
            _1of6 => "-1/6",
            _2of6 => "-2/6",
            _3of6 => "-3/6",
            _4of6 => "-4/6",
            _5of6 => "-5/6",
            _1of12 => "-1/12",
            _2of12 => "-2/12",
            _3of12 => "-3/12",
            _4of12 => "-4/12",
            _5of12 => "-5/12",
            _6of12 => "-6/12",
            _7of12 => "-7/12",
            _8of12 => "-8/12",
            _9of12 => "-9/12",
            _10of12 => "-10/12",
            _11of12 => "-11/12",
            Full => "-full",
            Screen => "-screen",
            Min => "-min",
            Max => "-max",
        }
        .into()
    }
}

pub enum MinSize {
    _0,
    Full,
    Min,
    Max,
}

impl From<MinSize> for String {
    fn from(size: MinSize) -> Self {
        use MinSize::*;

        match size {
            _0 => "-0",
            Full => "-full",
            Min => "-min",
            Max => "-max",
        }
        .into()
    }
}

pub enum MaxSize {
    _0,
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    _2xl,
    _3xl,
    _4xl,
    _5xl,
    _6xl,
    _7xl,
    Full,
    Min,
    Max,
    Prose,
    ScreenSm,
    ScreenMd,
    ScreenLg,
    ScreenXl,
    Screen2xl,
}

impl From<MaxSize> for String {
    fn from(size: MaxSize) -> Self {
        use MaxSize::*;

        match size {
            _0 => "-0",
            None => "-none",
            Xs => "-xs",
            Sm => "-sm",
            Md => "-md",
            Lg => "-lg",
            Xl => "-xl",
            _2xl => "-2xl",
            _3xl => "-3xl",
            _4xl => "-4xl",
            _5xl => "-5xl",
            _6xl => "-6xl",
            _7xl => "-7xl",
            Full => "-full",
            Min => "-min",
            Max => "-max",
            Prose => "-prose",
            ScreenSm => "-screen-sm",
            ScreenMd => "-screen-md",
            ScreenLg => "-screen-lg",
            ScreenXl => "-screen-xl",
            Screen2xl => "-screen-2xl",
        }
        .into()
    }
}
