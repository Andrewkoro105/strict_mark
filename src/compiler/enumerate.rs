use crate::{
    compiler::{BaseCompiler, BaseCompilerDataRef, Compile, Compiler, Ctx, Error, ResultCompile},
    data::{self, EnumerateType},
    rdocx_decl::{
        self,
        paragraph::{ParagraphListLevel, ParagraphListLevelStyle},
        document::StyleAdd,
    },
};
use serde::{Deserialize, Serialize};

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
        Option<ResultCompile>,
    > for (Enumerate, &Compiler)
{
    fn compile(
        &self,
        ast: &data::Enumerate<data::ParseData>,
        ctx: &Ctx,
        cash: &mut super::Cash,
    ) -> Option<ResultCompile> {
        BaseCompilerDataRef::new(
            &Data {
                enumerate_type: ast.enumerate_type.clone(),
            },
            ctx,
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
                                ResultCompile,
                            >>::compile(self.1, ast, ctx, cash)
                        })
                        .fold(
                            (Vec::new(), None, Vec::new()),
                            |(mut acc_contents, style1, mut acc_errors),
                             (mut contents, style2, errors): ResultCompile| {
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
                                                        style: style.rdocx_style_name.clone(),
                                                        level: 1,
                                                    })
                                            }
                                        }
                                    }
                                });
                                acc_contents.extend(contents);
                                acc_errors.extend(errors);
                                (acc_contents, style1.add(style2), acc_errors)
                            },
                        )
                })
        })
    }
}
