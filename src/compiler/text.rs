use crate::{compiler::{Compile, Ctx}, data::TextVariants, rdocx_decl::run::{self, Run}};

pub struct Text;

impl Compile<TextVariants, Run> for Text {
    fn compile(&self, ast: &TextVariants, _ctx: &Ctx, _cash: &mut super::Cash) -> Run {
        match ast {
            TextVariants::PhantomNewLine => Run {
                contents: vec![run::Content::Text("".into())],
                style: run::Style::default(),
            },
            TextVariants::Text(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default(),
            },
            TextVariants::Bold(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().bold(Some(true)),
            },
            TextVariants::Italic(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().italic(Some(true)),
            },
            TextVariants::Underlined(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().underline_style(Some(run::UnderlineStyle::Single)),
            },
            TextVariants::StruckThrough(text) => Run {
                contents: vec![run::Content::Text(text.clone())],
                style: run::Style::default().strike(Some(true)),
            },
            TextVariants::UnbreakableText(_text) => todo!("Not supported: {ast:#?}"),
            TextVariants::Link(_items) => todo!("Not supported: {ast:#?}"),
            TextVariants::InlineFormula(_formula) => todo!("Not supported: {ast:#?}"),
        }
    }
}
