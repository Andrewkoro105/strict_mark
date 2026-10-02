use serde::{Deserialize, Serialize};

use crate::rdocx_decl::utils::{Color, Length};

//todo: This element is taken from rdocs; later, it needs to be changed in rdocs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Single,
    Thick,
    Double,
    Dotted,
    Dashed,
    DotDash,
    Wave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    pub style: BorderStyle,
    pub size: Length,
    pub space_points: u32,
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Borders {
    All {
        style: BorderStyle,
        size: Length,
        color: Color,
    },
    Bottom(Border),
}

impl Into<rdocx::BorderStyle> for BorderStyle {
    fn into(self) -> rdocx::BorderStyle {
        match self {
            BorderStyle::None => rdocx::BorderStyle::None,
            BorderStyle::Single => rdocx::BorderStyle::Single,
            BorderStyle::Thick => rdocx::BorderStyle::Thick,
            BorderStyle::Double => rdocx::BorderStyle::Double,
            BorderStyle::Dotted => rdocx::BorderStyle::Dotted,
            BorderStyle::Dashed => rdocx::BorderStyle::Dashed,
            BorderStyle::DotDash => rdocx::BorderStyle::DotDash,
            BorderStyle::Wave => rdocx::BorderStyle::Wave,
        }
    }
}
