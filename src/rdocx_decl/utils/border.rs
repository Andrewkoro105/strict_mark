use serde::{Deserialize, Serialize};

use crate::rdocx_decl::utils::{Color, Length};

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
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

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Edge {
    /// The top edge.
    Top,
    /// The bottom edge.
    Bottom,
    /// The left edge.
    Left,
    /// The right edge.
    Right,
    /// The edge drawn between consecutive paragraphs that share a border.
    Between,
    /// The bar edge drawn beside the paragraph.
    Bar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    pub style: BorderStyle,
    pub size: Length,
    pub color: Color,
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

impl Into<rdocx::ParagraphBorderEdge> for Edge {
    fn into(self) -> rdocx::ParagraphBorderEdge {
        match self {
            Edge::Top => rdocx::ParagraphBorderEdge::Top,
            Edge::Bottom => rdocx::ParagraphBorderEdge::Bottom,
            Edge::Left => rdocx::ParagraphBorderEdge::Left,
            Edge::Right => rdocx::ParagraphBorderEdge::Right,
            Edge::Between => rdocx::ParagraphBorderEdge::Between,
            Edge::Bar => rdocx::ParagraphBorderEdge::Bar,
        }
    }
}