use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

use crate::rdocx_decl::{paragraph::Alignment, utils::Length};

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListNumberFormat {
    Decimal,
    UpperRoman,
    LowerRoman,
    UpperLetter,
    LowerLetter,
    Ordinal,
    CardinalText,
    OrdinalText,
    Hex,
    Chicago,
    IdeographDigital,
    JapaneseCounting,
    Aiueo,
    Iroha,
    DecimalFullWidth,
    DecimalHalfWidth,
    JapaneseLegal,
    JapaneseDigitalTenThousand,
    DecimalEnclosedCircle,
    DecimalFullWidth2,
    AiueoFullWidth,
    IrohaFullWidth,
    DecimalZero,
    Bullet,
    Ganada,
    Chosung,
    DecimalEnclosedFullstop,
    DecimalEnclosedParen,
    DecimalEnclosedCircleChinese,
    IdeographEnclosedCircle,
    IdeographTraditional,
    IdeographZodiac,
    IdeographZodiacTraditional,
    TaiwaneseCounting,
    IdeographLegalTraditional,
    TaiwaneseCountingThousand,
    TaiwaneseDigital,
    ChineseCounting,
    ChineseLegalSimplified,
    ChineseCountingThousand,
    KoreanDigital,
    KoreanCounting,
    KoreanLegal,
    KoreanDigital2,
    Hebrew1,
    ArabicAlpha,
    Hebrew2,
    ArabicAbjad,
    HindiVowels,
    HindiConsonants,
    HindiNumbers,
    HindiCounting,
    ThaiLetters,
    ThaiNumbers,
    ThaiCounting,
    VietnameseCounting,
    NumberInDash,
    RussianLower,
    RussianUpper,
    None,
    /// A producer-defined format retained across inspection and mutation.
    Other(String),
}

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListLevelSuffix {
    Tab,
    Space,
    Nothing,
}

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListLevelRestart {
    /// Continue across every more-significant level.
    Never,
    /// Restart after the given zero-based, more-significant level.
    After(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ListLevelStyleId {
    Name(String),
    Num(usize),
}

//todo: Add to marker_properties in the future.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLevel {
    format: ListNumberFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    level_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    suffix: Option<ListLevelSuffix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    alignment: Option<Alignment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    indent_left: Option<Length>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    indent_hanging: Option<Length>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    indent_first_line: Option<Length>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    legal_numbering: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    restart: Option<ListLevelRestart>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    template_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tentative: Option<bool>,
}

impl From<ListLevel> for rdocx::ListLevel {
    fn from(val: ListLevel) -> Self {
        let mut result = rdocx::ListLevel::new(val.format.into());
        if let Some(start) = val.start {
            result = result.start(start);
        }
        if let Some(level_text) = val.level_text {
            result = result.level_text(level_text);
        }
        if let Some(suffix) = val.suffix {
            result = result.suffix(suffix.into());
        }
        if let Some(alignment) = val.alignment {
            result = result.alignment(alignment.into());
        }
        result = result.indentation(
            val.indent_left
                .map(Length::into_rdocx)
                .map(rdocx::Length::as_twips),
            val.indent_hanging
                .map(Length::into_rdocx)
                .map(rdocx::Length::as_twips),
            val.indent_first_line
                .map(Length::into_rdocx)
                .map(rdocx::Length::as_twips),
        );
        if let Some(legal_numbering) = val.legal_numbering {
            result = result.legal_numbering(legal_numbering);
        }
        if let Some(restart) = val.restart {
            result = result.restart(restart.into());
        }
        if let Some(template_code) = val.template_code {
            result = result.template_code(template_code);
        }
        if let Some(tentative) = val.tentative {
            result = result.tentative(tentative);
        }
        result
    }
}

impl From<ListLevelSuffix> for rdocx::ListLevelSuffix {
    fn from(val: ListLevelSuffix) -> Self {
        match val {
            ListLevelSuffix::Tab => rdocx::ListLevelSuffix::Tab,
            ListLevelSuffix::Space => rdocx::ListLevelSuffix::Space,
            ListLevelSuffix::Nothing => rdocx::ListLevelSuffix::Nothing,
        }
    }
}

impl From<ListLevelRestart> for rdocx::ListLevelRestart {
    fn from(val: ListLevelRestart) -> Self {
        match val {
            ListLevelRestart::Never => rdocx::ListLevelRestart::Never,
            ListLevelRestart::After(after) => rdocx::ListLevelRestart::After(after),
        }
    }
}

impl From<ListNumberFormat> for rdocx::ListNumberFormat {
    fn from(val: ListNumberFormat) -> Self {
        match val {
            ListNumberFormat::Decimal => rdocx::ListNumberFormat::Decimal,
            ListNumberFormat::UpperRoman => rdocx::ListNumberFormat::UpperRoman,
            ListNumberFormat::LowerRoman => rdocx::ListNumberFormat::LowerRoman,
            ListNumberFormat::UpperLetter => rdocx::ListNumberFormat::UpperLetter,
            ListNumberFormat::LowerLetter => rdocx::ListNumberFormat::LowerLetter,
            ListNumberFormat::Ordinal => rdocx::ListNumberFormat::Ordinal,
            ListNumberFormat::CardinalText => rdocx::ListNumberFormat::CardinalText,
            ListNumberFormat::OrdinalText => rdocx::ListNumberFormat::OrdinalText,
            ListNumberFormat::Hex => rdocx::ListNumberFormat::Hex,
            ListNumberFormat::Chicago => rdocx::ListNumberFormat::Chicago,
            ListNumberFormat::IdeographDigital => rdocx::ListNumberFormat::IdeographDigital,
            ListNumberFormat::JapaneseCounting => rdocx::ListNumberFormat::JapaneseCounting,
            ListNumberFormat::Aiueo => rdocx::ListNumberFormat::Aiueo,
            ListNumberFormat::Iroha => rdocx::ListNumberFormat::Iroha,
            ListNumberFormat::DecimalFullWidth => rdocx::ListNumberFormat::DecimalFullWidth,
            ListNumberFormat::DecimalHalfWidth => rdocx::ListNumberFormat::DecimalHalfWidth,
            ListNumberFormat::JapaneseLegal => rdocx::ListNumberFormat::JapaneseLegal,
            ListNumberFormat::JapaneseDigitalTenThousand => {
                rdocx::ListNumberFormat::JapaneseDigitalTenThousand
            }
            ListNumberFormat::DecimalEnclosedCircle => {
                rdocx::ListNumberFormat::DecimalEnclosedCircle
            }
            ListNumberFormat::DecimalFullWidth2 => rdocx::ListNumberFormat::DecimalFullWidth2,
            ListNumberFormat::AiueoFullWidth => rdocx::ListNumberFormat::AiueoFullWidth,
            ListNumberFormat::IrohaFullWidth => rdocx::ListNumberFormat::IrohaFullWidth,
            ListNumberFormat::DecimalZero => rdocx::ListNumberFormat::DecimalZero,
            ListNumberFormat::Bullet => rdocx::ListNumberFormat::Bullet,
            ListNumberFormat::Ganada => rdocx::ListNumberFormat::Ganada,
            ListNumberFormat::Chosung => rdocx::ListNumberFormat::Chosung,
            ListNumberFormat::DecimalEnclosedFullstop => {
                rdocx::ListNumberFormat::DecimalEnclosedFullstop
            }
            ListNumberFormat::DecimalEnclosedParen => rdocx::ListNumberFormat::DecimalEnclosedParen,
            ListNumberFormat::DecimalEnclosedCircleChinese => {
                rdocx::ListNumberFormat::DecimalEnclosedCircleChinese
            }
            ListNumberFormat::IdeographEnclosedCircle => {
                rdocx::ListNumberFormat::IdeographEnclosedCircle
            }
            ListNumberFormat::IdeographTraditional => rdocx::ListNumberFormat::IdeographTraditional,
            ListNumberFormat::IdeographZodiac => rdocx::ListNumberFormat::IdeographZodiac,
            ListNumberFormat::IdeographZodiacTraditional => {
                rdocx::ListNumberFormat::IdeographZodiacTraditional
            }
            ListNumberFormat::TaiwaneseCounting => rdocx::ListNumberFormat::TaiwaneseCounting,
            ListNumberFormat::IdeographLegalTraditional => {
                rdocx::ListNumberFormat::IdeographLegalTraditional
            }
            ListNumberFormat::TaiwaneseCountingThousand => {
                rdocx::ListNumberFormat::TaiwaneseCountingThousand
            }
            ListNumberFormat::TaiwaneseDigital => rdocx::ListNumberFormat::TaiwaneseDigital,
            ListNumberFormat::ChineseCounting => rdocx::ListNumberFormat::ChineseCounting,
            ListNumberFormat::ChineseLegalSimplified => {
                rdocx::ListNumberFormat::ChineseLegalSimplified
            }
            ListNumberFormat::ChineseCountingThousand => {
                rdocx::ListNumberFormat::ChineseCountingThousand
            }
            ListNumberFormat::KoreanDigital => rdocx::ListNumberFormat::KoreanDigital,
            ListNumberFormat::KoreanCounting => rdocx::ListNumberFormat::KoreanCounting,
            ListNumberFormat::KoreanLegal => rdocx::ListNumberFormat::KoreanLegal,
            ListNumberFormat::KoreanDigital2 => rdocx::ListNumberFormat::KoreanDigital2,
            ListNumberFormat::Hebrew1 => rdocx::ListNumberFormat::Hebrew1,
            ListNumberFormat::ArabicAlpha => rdocx::ListNumberFormat::ArabicAlpha,
            ListNumberFormat::Hebrew2 => rdocx::ListNumberFormat::Hebrew2,
            ListNumberFormat::ArabicAbjad => rdocx::ListNumberFormat::ArabicAbjad,
            ListNumberFormat::HindiVowels => rdocx::ListNumberFormat::HindiVowels,
            ListNumberFormat::HindiConsonants => rdocx::ListNumberFormat::HindiConsonants,
            ListNumberFormat::HindiNumbers => rdocx::ListNumberFormat::HindiNumbers,
            ListNumberFormat::HindiCounting => rdocx::ListNumberFormat::HindiCounting,
            ListNumberFormat::ThaiLetters => rdocx::ListNumberFormat::ThaiLetters,
            ListNumberFormat::ThaiNumbers => rdocx::ListNumberFormat::ThaiNumbers,
            ListNumberFormat::ThaiCounting => rdocx::ListNumberFormat::ThaiCounting,
            ListNumberFormat::VietnameseCounting => rdocx::ListNumberFormat::VietnameseCounting,
            ListNumberFormat::NumberInDash => rdocx::ListNumberFormat::NumberInDash,
            ListNumberFormat::RussianLower => rdocx::ListNumberFormat::RussianLower,
            ListNumberFormat::RussianUpper => rdocx::ListNumberFormat::RussianUpper,
            ListNumberFormat::None => rdocx::ListNumberFormat::None,
            ListNumberFormat::Other(other) => rdocx::ListNumberFormat::Other(other),
        }
    }
}

impl ListLevelStyleId {
    pub fn unique() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        ListLevelStyleId::Num(COUNTER.fetch_add(1, Ordering::Relaxed) + 1)
    }
}