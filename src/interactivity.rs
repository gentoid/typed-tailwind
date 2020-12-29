use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct AppearanceNone;

impl From<&AppearanceNone> for String {
    fn from(_: &AppearanceNone) -> String {
        "appearence-none".into()
    }
}

#[derive(FocusState)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Outline(pub OutlineVal);

impl From<&Outline> for String {
    fn from(outline: &Outline) -> String {
        match outline {
            Outline(inner) => format!("outline{}", String::from(inner))
        }
    }
}

pub enum OutlineVal {
    None,
    White,
    Black,
}

impl From<&OutlineVal> for String {
    fn from(outline: &OutlineVal) -> String {
        use OutlineVal::*;

        match outline {
            OutlineVal::None => "-none",
            White => "-white",
            Black => "-black",
        }.into()
    }
}
