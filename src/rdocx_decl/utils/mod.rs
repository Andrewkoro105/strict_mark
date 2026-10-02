#[macro_use]
pub mod to_rdocx_static_dispatch;
pub mod border;
pub mod tab;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    Hex(String),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Length {
    Emu(i64),
    Inches(f64),
    Cm(f64),
    Mm(f64),
    Pt(f64),
    Twips(i32),
}

//todo: This element is taken from rdocs; later, it needs to be changed in rdocs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectionBreak {
    /// Start a new section on the next page.
    NextPage,
    /// Start a new section on the same page (continuous).
    Continuous,
    /// Start a new section on the next even-numbered page.
    EvenPage,
    /// Start a new section on the next odd-numbered page.
    OddPage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionPageSize {
    Landscape,
    Portrait,
    Size { width: Length, height: Length },
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Hex(hex) => hex.clone(),
            Color::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
        }
    }
}

impl Length {
    pub fn into_rdocx(self) -> rdocx::Length {
        self.into()
    }
}

impl Into<rdocx::Length> for Length {
    fn into(self) -> rdocx::Length {
        match self {
            Length::Emu(emu) => rdocx::Length::emu(emu),
            Length::Inches(inches) => rdocx::Length::inches(inches),
            Length::Cm(cm) => rdocx::Length::cm(cm),
            Length::Mm(mm) => rdocx::Length::mm(mm),
            Length::Pt(pt) => rdocx::Length::pt(pt),
            Length::Twips(twips) => rdocx::Length::twips(twips),
        }
    }
}

impl Into<rdocx::SectionBreak> for SectionBreak {
    fn into(self) -> rdocx::SectionBreak {
        match self {
            SectionBreak::NextPage => rdocx::SectionBreak::NextPage,
            SectionBreak::Continuous => rdocx::SectionBreak::Continuous,
            SectionBreak::EvenPage => rdocx::SectionBreak::EvenPage,
            SectionBreak::OddPage => rdocx::SectionBreak::OddPage,
        }
    }
}