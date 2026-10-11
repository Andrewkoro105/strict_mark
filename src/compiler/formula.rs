use crate::{
    compiler::{BaseCompiler, Compile, Ctx},
    data,
    rdocx_decl::{
        self,
        equation::{Equation, EquationType},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub rdocx_style: rdocx_decl::paragraph::Style,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {}

pub type Formula = BaseCompiler<Style, Data>;

impl Compile<data::Formula> for Formula {
    fn compile(
        &self,
        ast: &data::Formula,
        _ctx: &Ctx,
        _cash: &mut super::Cash,
    ) -> Option<Vec<rdocx_decl::document::Content>> {
        Some(self.style.compile(ast, &self.data).unwrap_or_else(|style| {
            vec![
                rdocx_decl::paragraph::Paragraph {
                    contents: vec![
                        Equation {
                            latex: ast.formula.clone(),
                            equation_type: EquationType::Display,
                        }
                        .into(),
                    ],
                    style: style.rdocx_style,
                }
                .into(),
            ]
        }))
    }
}


