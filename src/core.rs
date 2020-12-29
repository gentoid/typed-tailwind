pub trait ScreenSizeTrait {
    fn screen(self, screen: Screen) -> ScreenSize;
}

pub struct ScreenSize(pub Screen, pub Box<dyn ScreenSizeTrait>);

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
