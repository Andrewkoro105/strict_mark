use crate::{
    compiler::{BaseCompiler, Compile, text::Text},
    data::{self, ParagraphType},
    rdocx_decl::{self},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub rdocx_style: rdocx_decl::paragraph::Style,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub paragraph_type: ParagraphType,
}

pub type Paragraph = BaseCompiler<Style, Data>;

impl Compile<data::Paragraph> for Paragraph {
    fn compile(
        &self,
        ast: &data::Paragraph,
        cash: &mut super::Cash,
    ) -> Option<Vec<rdocx_decl::document::Content>> {
        self
            .data
            .as_ref()
            .map(|data| ast.paragraph_type == data.paragraph_type)
            .unwrap_or(true)
        .then(|| {
            self.style.compile(ast, &self.data).unwrap_or_else(|style| {
                vec![
                    rdocx_decl::paragraph::Paragraph {
                        contents: ast
                            .text
                            .iter()
                            .map(|text| (Text).compile(text, cash).into())
                            .collect(),
                        style: style.rdocx_style,
                    }
                    .into(),
                ]
            })
        })
    }
}
