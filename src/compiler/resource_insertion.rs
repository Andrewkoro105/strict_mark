use std::path::PathBuf;

use crate::{
    compiler::{BaseCompiler, BaseCompilerDataRef, Compile, Ctx, text::Text},
    data,
    rdocx_decl::{
        self,
        run::{self, PictureId, Run},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub rdocx_style_for_image: rdocx_decl::paragraph::Style,
    pub rdocx_style_for_caption: rdocx_decl::paragraph::Style,
    pub dpi: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data;

pub type ResourceInsertion = BaseCompiler<Style, Data>;

impl
    Compile<
        data::ResourceInsertion,
        Option<(Vec<rdocx_decl::document::Content>, (PictureId, PathBuf))>,
    > for ResourceInsertion
{
    fn compile(
        &self,
        ast: &data::ResourceInsertion,
        ctx: &Ctx,
        cash: &mut super::Cash,
    ) -> Option<(Vec<rdocx_decl::document::Content>, (PictureId, PathBuf))> {
        BaseCompilerDataRef::new(&Data, ctx)
            .eq(&self.data)
            .then(|| {
                let id = PictureId::unique();
                (
                    self.style.compile(ast, &self.data).unwrap_or_else(|style| {
                        vec![
                            rdocx_decl::paragraph::Paragraph {
                                contents: vec![
                                    Run {
                                        contents: vec![run::Content::Picture {
                                            id: id.clone(),
                                            dpi: style.dpi
                                        }],
                                        style: Default::default(),
                                    }
                                    .into(),
                                ],
                                style: style.rdocx_style_for_image,
                            }
                            .into(),
                            rdocx_decl::paragraph::Paragraph {
                                contents: ast
                                    .caption
                                    .iter()
                                    .map(|text| (Text).compile(text, ctx, cash))
                                    .collect(),
                                style: style.rdocx_style_for_caption,
                            }
                            .into(),
                        ]
                    }),
                    (id, ast.path.clone()),
                )
            })
    }
}
