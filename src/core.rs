#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

pub trait ScreenSizeTrait {
    fn screen(&self, screen: Screen) -> ScreenSize;
    fn to_string(&self) -> String;
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct ScreenSize<'a>(pub Screen, pub &'a (dyn ScreenSizeTrait));

impl<'a> From<&ScreenSize<'a>> for String {
    fn from(size: &ScreenSize) -> Self {
        match size {
            ScreenSize(screen, data) => {
                format!("{}{}", String::from(screen), data.to_string())
            }
        }
    }
}

pub enum Screen {
    Sm,
    Md,
    Lg,
    Xl,
    _2xl,
}

impl From<&Screen> for String {
    fn from(bp: &Screen) -> Self {
        use Screen::*;

        match bp {
            Sm => "sm:",
            Md => "md:",
            Lg => "lg:",
            Xl => "xl:",
            _2xl => "2xl:",
        }
        .into()
    }
}

pub enum ColorTone {
    _50,
    _100,
    _200,
    _300,
    _400,
    _500,
    _600,
    _700,
    _800,
    _900,
}

impl From<ColorTone> for String {
    fn from(tone: ColorTone) -> Self {
        use ColorTone::*;

        match tone {
            _50 => "-50",
            _100 => "-100",
            _200 => "-200",
            _300 => "-300",
            _400 => "-400",
            _500 => "-500",
            _600 => "-600",
            _700 => "-700",
            _800 => "-800",
            _900 => "-900",
        }
        .into()
    }
}

pub enum Color {
    Transparent,
    Current,
    Black,
    White,
    BlueGray(ColorTone),
    CoolGray(ColorTone),
    Gray(ColorTone),
    TrueGray(ColorTone),
    WarmGray(ColorTone),
    Red(ColorTone),
    Orange(ColorTone),
    Amber(ColorTone),
    Yellow(ColorTone),
    Lime(ColorTone),
    Green(ColorTone),
    Emerald(ColorTone),
    Teal(ColorTone),
    Cyan(ColorTone),
    LightBlue(ColorTone),
    Blue(ColorTone),
    Indigo(ColorTone),
    Violet(ColorTone),
    Purple(ColorTone),
    Fuchsia(ColorTone),
    Pink(ColorTone),
    Rose(ColorTone),
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        use Color::*;

        match color {
            Transparent => "-transparent".into(),
            Current => "-current".into(),
            Black => "-black".into(),
            White => "-white".into(),
            BlueGray(tone) => format!("-blueGray{}", String::from(tone)),
            CoolGray(tone) => format!("-coolGray{}", String::from(tone)),
            Gray(tone) => format!("-gray{}", String::from(tone)),
            TrueGray(tone) => format!("-trueGray{}", String::from(tone)),
            WarmGray(tone) => format!("-warmGray{}", String::from(tone)),
            Red(tone) => format!("-red{}", String::from(tone)),
            Orange(tone) => format!("-orange{}", String::from(tone)),
            Amber(tone) => format!("-amber{}", String::from(tone)),
            Yellow(tone) => format!("-yellow{}", String::from(tone)),
            Lime(tone) => format!("-lime{}", String::from(tone)),
            Green(tone) => format!("-green{}", String::from(tone)),
            Emerald(tone) => format!("-emerald{}", String::from(tone)),
            Teal(tone) => format!("-teal{}", String::from(tone)),
            Cyan(tone) => format!("-cyan{}", String::from(tone)),
            LightBlue(tone) => format!("-lightBlue{}", String::from(tone)),
            Blue(tone) => format!("-blue{}", String::from(tone)),
            Indigo(tone) => format!("-indigo{}", String::from(tone)),
            Violet(tone) => format!("-violet{}", String::from(tone)),
            Purple(tone) => format!("-purple{}", String::from(tone)),
            Fuchsia(tone) => format!("-fuchsia{}", String::from(tone)),
            Pink(tone) => format!("-pink{}", String::from(tone)),
            Rose(tone) => format!("-rose{}", String::from(tone)),
        }
    }
}
