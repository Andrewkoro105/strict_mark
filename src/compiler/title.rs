use crate::{
    compiler::{BaseCompiler, BaseCompilerDataRef, Compile, Ctx, text::Text},
    data, rdocx_decl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub rdocx_style: rdocx_decl::paragraph::Style,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    pub level: usize,
}

pub type Title = BaseCompiler<Style, Data>;

impl Compile<data::Title> for Title {
    fn compile(
        &self,
        ast: &data::Title,
        ctx: &Ctx,
        cash: &mut super::Cash,
    ) -> Option<Vec<rdocx_decl::document::Content>> {
        BaseCompilerDataRef::new(
            &Data {
                level: ast.level,
            },
            &ctx,
        )
        .eq(&self.data)
        .then(|| {
            self.style.compile(ast, &self.data).unwrap_or_else(|style| {
                vec![
                    rdocx_decl::paragraph::Paragraph {
                        contents: ast
                            .text
                            .iter()
                            .map(|text| (Text).compile(text, ctx, cash).into())
                            .collect(),
                        style: style.rdocx_style,
                    }
                    .into(),
                ]
            })
        })
    }
}
