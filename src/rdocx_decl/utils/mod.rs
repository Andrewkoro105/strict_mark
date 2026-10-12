#[macro_use]
pub mod to_rdocx_static_dispatch;
pub mod border;
pub mod tab;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Hex(String),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Length {
    Emu(i64),
    Inches(f64),
    Cm(f64),
    Mm(f64),
    Pt(f64),
    Twips(i32),
}

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SectionPageSize {
    Landscape,
    Portrait,
    Size { width: Length, height: Length },
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(hex) => hex.fmt(f),
            Color::Rgb(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b).fmt(f),
        }
    }
}

impl Length {
    pub fn into_rdocx(self) -> rdocx::Length {
        self.into()
    }
}

impl From<Length> for rdocx::Length {
    fn from(val: Length) -> Self {
        match val {
            Length::Emu(emu) => rdocx::Length::emu(emu),
            Length::Inches(inches) => rdocx::Length::inches(inches),
            Length::Cm(cm) => rdocx::Length::cm(cm),
            Length::Mm(mm) => rdocx::Length::mm(mm),
            Length::Pt(pt) => rdocx::Length::pt(pt),
            Length::Twips(twips) => rdocx::Length::twips(twips),
        }
    }
}

impl From<SectionBreak> for rdocx::SectionBreak {
    fn from(val: SectionBreak) -> Self {
        match val {
            SectionBreak::NextPage => rdocx::SectionBreak::NextPage,
            SectionBreak::Continuous => rdocx::SectionBreak::Continuous,
            SectionBreak::EvenPage => rdocx::SectionBreak::EvenPage,
            SectionBreak::OddPage => rdocx::SectionBreak::OddPage,
        }
    }
}