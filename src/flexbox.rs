pub enum ItemAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl From<ItemAlign> for String {
    fn from(position: ItemAlign) -> Self {
        use ItemAlign::*;

        match position {
            Start => "-start",
            End => "-end",
            Center => "-center",
            Baseline => "-baseline",
            Stretch => "-stretch",
        }
        .into()
    }
}

pub enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

impl From<JustifyContent> for String {
    fn from(position: JustifyContent) -> Self {
        use JustifyContent::*;

        match position {
            Start => "-start",
            End => "-end",
            Center => "-center",
            Between => "-between",
            Around => "-around",
            Evenly => "-evenly,",
        }
        .into()
    }
}
