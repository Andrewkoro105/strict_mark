use serde::{Deserialize, Serialize};

use crate::rdocx_decl::utils::Length;

//todo: This element is taken from rdocs; later, it needs to be changed in rdocs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabAlignment {
    Left,
    Center,
    Right,
    Decimal,
}

//todo: This element is taken from rdocs; later, it needs to be changed in rdocs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabLeader {
    None,
    Dot,
    Hyphen,
    Underscore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TabStop {
    TabStop {
        alignment: TabAlignment,
        position: Length,
    },
    TabStopWithLeader{
        alignment: TabAlignment,
        position: Length,
        leader: TabLeader,
    }
}

impl Into<rdocx::TabAlignment> for TabAlignment {
    fn into(self) -> rdocx::TabAlignment {
        match self {
            TabAlignment::Left => rdocx::TabAlignment::Left,
            TabAlignment::Center => rdocx::TabAlignment::Center,
            TabAlignment::Right => rdocx::TabAlignment::Right,
            TabAlignment::Decimal => rdocx::TabAlignment::Decimal,
        }
    }
}

impl Into<rdocx::TabLeader> for TabLeader {
    fn into(self) -> rdocx::TabLeader {
        match self {
            TabLeader::None => rdocx::TabLeader::None,
            TabLeader::Dot => rdocx::TabLeader::Dot,
            TabLeader::Hyphen => rdocx::TabLeader::Hyphen,
            TabLeader::Underscore => rdocx::TabLeader::Underscore,
        }
    }
}