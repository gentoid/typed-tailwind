pub trait ScreenSizeTrait {
    fn screen(&self, screen: Screen) -> ScreenSize;
    fn to_string(&self) -> String;
}

pub struct ScreenSize<'a>(pub Screen, pub &'a (dyn ScreenSizeTrait));

impl From<ScreenSize<'_>> for String {
    fn from(size: ScreenSize) -> Self {
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

impl From<Screen> for String {
    fn from(bp: Screen) -> Self {
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
