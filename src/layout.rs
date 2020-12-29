use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[derive(ScreenSize)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Invisible;

impl From<&Invisible> for String {
    fn from(_: &Invisible) -> String {
        "invisible".into()
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Overflow(pub OverflowSide, pub OverflowType);

impl From<&Overflow> for String {
    fn from(overflow: &Overflow) -> String {
        match overflow {
            Overflow(side, o_type) => {
                format!("overflow{}{}", String::from(side), String::from(o_type))
            }
        }
    }
}

pub enum OverflowType {
    Auto,
    Hidden,
    Visible,
    Scroll,
}

impl From<&OverflowType> for String {
    fn from(of_type: &OverflowType) -> Self {
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

impl From<&OverflowSide> for String {
    fn from(side: &OverflowSide) -> Self {
        use OverflowSide::*;

        match side {
            All => "",
            X => "-x",
            Y => "-y",
        }
        .into()
    }
}
