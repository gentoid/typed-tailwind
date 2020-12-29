#[cfg(feature = "seed_support")]
use seed::virtual_dom::to_classes::ToClasses;
#[cfg(feature = "seed_support")]
use typed_tailwind_seed_derive::*;

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Flex;

impl From<&Flex> for String {
    fn from(_: &Flex) -> Self {
        "flex".into()
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct FlexRow;

impl From<&FlexRow> for String {
    fn from(_: &FlexRow) -> String {
        "flex-row".into()
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct FlexRowReverse;

impl From<&FlexRowReverse> for String {
    fn from(_: &FlexRowReverse) -> String {
        "flex-row-reverse".into()
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct FlexCol;

impl From<&FlexCol> for String {
    fn from(_: &FlexCol) -> String {
        "flex-col".into()
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct FlexColReverse;

impl From<&FlexColReverse> for String {
    fn from(_: &FlexColReverse) -> String {
        "flex-col-reverse".into()
    }
}

#[cfg_attr(feature = "seed_support", derive(ToSeedClass))]
pub struct Items(pub ItemAlign);

impl From<&Items> for String {
    fn from(items: &Items) -> String {
        match items {
            Items(align) => format!("items{}", String::from(align))
        }
    }
}


pub enum ItemAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl From<&ItemAlign> for String {
    fn from(position: &ItemAlign) -> Self {
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
