use crate::core::*;
use typed_tailwind_derive::*;

#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct BoxShadow(pub Shadow);

impl From<&BoxShadow> for String {
    fn from(box_shadow: &BoxShadow) -> String {
        match box_shadow {
            BoxShadow(inner) => format!("shadow{}", String::from(inner))
        }
    }
}

pub enum Shadow {
    Sm,
    Default,
    Md,
    Lg,
    Xl,
    _2xl,
    Inner,
    None,
}

impl From<&Shadow> for String {
    fn from(shadow: &Shadow) -> String {
        use Shadow::*;

        match shadow {
            Sm => "-sm",
            Default => "",
            Md => "-md",
            Lg => "-lg",
            Xl => "-xl",
            _2xl => "-2xl",
            Inner => "-inner",
            Shadow::None => "-none",
        }.into()
    }
}
