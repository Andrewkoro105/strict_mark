use crate::{compiler::{Base, Compile, text::Text}, data, rdocx_decl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub rdocx_style: rdocx_decl::paragraph::Style,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub level: usize,
}

pub type Title = Base<Style, Data>;

impl Compile<data::Title> for Title {
    fn compile(
        &self,
        ast: &data::Title,
        cash: &mut super::Cash,
    ) -> Option<Vec<rdocx_decl::document::Content>> {
        (ast.level == self.data.level).then(|| {
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
