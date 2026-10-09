use serde::{Deserialize, Serialize};

use crate::{
    compiler::{BaseCompiler, BaseCompilerDataRef, Compile, Compiler, Ctx, Error},
    data::{self, EnumerateType},
    rdocx_decl::{
        self,
        paragraph::{ParagraphListLevel, ParagraphListLevelStyle},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub rdocx_style_name: ParagraphListLevelStyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    enumerate_type: EnumerateType,
}

pub type Enumerate = BaseCompiler<Style, Data>;

impl
    Compile<
        data::Enumerate<data::ParseData>,
        Option<(Vec<rdocx_decl::document::Content>, Vec<Error>)>,
    > for (Enumerate, &Compiler)
{
    fn compile(
        &self,
        ast: &data::Enumerate<data::ParseData>,
        ctx: &Ctx,
        cash: &mut super::Cash,
    ) -> Option<(Vec<rdocx_decl::document::Content>, Vec<Error>)> {
        BaseCompilerDataRef::new(
            &Data {
                enumerate_type: ast.enumerate_type.clone(),
            },
            &ctx,
        )
        .eq(&self.0.data)
        .then(|| {
            self.0
                .style
                .compile(ast, &self.0.data)
                .unwrap_or_else(|style| {
                    ast.data
                        .iter()
                        .map(|ast| {
                            <Compiler as Compile<
                                data::ParseData,
                                (Vec<rdocx_decl::document::Content>, Vec<Error>),
                            >>::compile(&self.1, ast, &ctx, cash)
                        })
                        .fold(
                            (Vec::new(), Vec::new()),
                            |(mut acc_contents, mut acc_errors),
                             (mut contents, errors): (Vec<_>, _)| {
                                contents.iter_mut().enumerate().for_each(|(i, content)| {
                                    match content {
                                        rdocx_decl::document::Content::Paragraph(paragraph) => {
                                            let paragraph: &mut rdocx_decl::paragraph::Paragraph =
                                                paragraph;
                                            if let Some(list_level) =
                                                paragraph.style.list_level.as_mut()
                                            {
                                                list_level.level += 1;
                                            } else if i == 0 {
                                                paragraph.style.list_level =
                                                    Some(ParagraphListLevel {
                                                        stile: style.rdocx_style_name.clone(),
                                                        level: 1,
                                                    })
                                            }
                                        }
                                        _ => {}
                                    }
                                });
                                acc_contents.extend(contents);
                                acc_errors.extend(errors);
                                (acc_contents, acc_errors)
                            },
                        )
                })
        })
    }
}
