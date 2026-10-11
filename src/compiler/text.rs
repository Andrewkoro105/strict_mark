use crate::{compiler::{Compile, Ctx}, data::TextVariants, rdocx_decl::{equation::Equation, paragraph, run::{self, Run}}};

pub struct Text;

impl Compile<TextVariants, paragraph::Content> for Text {
    fn compile(&self, ast: &TextVariants, _ctx: &Ctx, _cash: &mut super::Cash) -> paragraph::Content {
        match ast {
            TextVariants::PhantomNewLine => Run {
                contents: vec![run::Content::Text(" ".into())],
                style: run::Style::default(),
            }.into(),
            TextVariants::Text(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default(),
            }.into(),
            TextVariants::Bold(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().bold(Some(true)),
            }.into(),
            TextVariants::Italic(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().italic(Some(true)),
            }.into(),
            TextVariants::Underlined(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().underline_style(Some(run::UnderlineStyle::Single)),
            }.into(),
            TextVariants::StruckThrough(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().strike(Some(true)),
            }.into(),
            TextVariants::UnbreakableText(_text) => todo!("Not supported: {ast:#?}"),
            TextVariants::Link(_items) => todo!("Not supported: {ast:#?}"),
            TextVariants::InlineFormula(formula) => Equation {
                latex: formula.clone(),
                equation_type: crate::rdocx_decl::equation::EquationType::Inline,
            }.into(),
        }
    }
}
