pub enum OverflowType {
    Auto,
    Hidden,
    Visible,
    Scroll,
}

impl From<OverflowType> for String {
    fn from(of_type: OverflowType) -> Self {
        use OverflowType::*;

        match of_type {
            Auto => "-auto",
            Hidden => "-hidden",
            Visible => "-visible",
            Scroll => "-scroll",
        }
        .into()
    }
}

pub enum OverflowSide {
    All,
    X,
    Y,
}

impl From<OverflowSide> for String {
    fn from(side: OverflowSide) -> Self {
        use OverflowSide::*;

        match side {
            All => "",
            X => "-x",
            Y => "-y",
        }.into()
    }
}
