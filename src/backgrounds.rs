use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[derive(FocusState, HoverState, ScreenSize)]
#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct BgColor(pub Color);

impl From<&BgColor> for String {
    fn from(bg_color: &BgColor) -> String {
        match bg_color {
            BgColor(color) => format!("bg{}", String::from(color))
        }
    }
}
