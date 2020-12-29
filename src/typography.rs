use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[derive(ScreenSize)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct TextAlign(pub AlignTextAt);

impl From<&TextAlign> for String {
    fn from(text_align: &TextAlign) -> String {
        match text_align {
            TextAlign(align) => format!("text{}", String::from(align))
        }
    }
}

#[derive(HoverState)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct TextColor(pub Color);

impl From<&TextColor> for String {
    fn from(text_color: &TextColor) -> String {
        match text_color {
            TextColor(color) => format!("text{}", String::from(color))
        }
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct FontWeight(pub FontWeightVal);

impl From<&FontWeight> for String {
    fn from(font_weight: &FontWeight) -> String {
        match font_weight {
            FontWeight(weight) => format!("fornt{}", String::from(weight))
        }
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct TextSize(pub TextSizeVal);

impl From<&TextSize> for String {
    fn from(text_size: &TextSize) -> String {
        match text_size {
            TextSize(size) => format!("text{}", String::from(size))
        }
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Leading(pub LineHeight);

impl From<&Leading> for String {
    fn from(leading: &Leading) -> String {
        match leading {
            Leading(line_height) => format!("leading{}", String::from(line_height))
        }
    }
}

pub enum AlignTextAt {
    Left,
    Center,
    Right,
    Justify,
}

impl From<&AlignTextAt> for String {
    fn from(size: &AlignTextAt) -> Self {
        use AlignTextAt::*;

        match size {
            Left => "-left",
            Center => "-center",
            Right => "-right",
            Justify => "-justify",
        }
        .into()
    }
}

pub enum TextSizeVal {
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

impl From<&TextSizeVal> for String {
    fn from(size: &TextSizeVal) -> Self {
        use TextSizeVal::*;

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

pub enum FontWeightVal {
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

impl From<&FontWeightVal> for String {
    fn from(weight: &FontWeightVal) -> Self {
        use FontWeightVal::*;

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

pub enum LineHeight {
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

impl From<&LineHeight> for String {
    fn from(leading: &LineHeight) -> Self {
        use LineHeight::*;

        match leading {
            _3 => "-3",
            _4 => "-4",
            _5 => "-5",
            _6 => "-6",
            _7 => "-7",
            _8 => "-8",
            _9 => "-9",
            _10 => "-10",
            LineHeight::None => "-none",
            Tight => "-tight",
            Snug => "-snug",
            Normal => "-normal",
            Relaxed => "-relaxed",
            Loose => "-loose",
        }
        .into()
    }
}
