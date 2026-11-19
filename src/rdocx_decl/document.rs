use std::{collections::HashMap, ffi::OsStr, io::Cursor, path::PathBuf};

use super::is_default;
use crate::rdocx_decl::{
    self, Cash, ToRdocx,
    list_level::{ListLevel, ListLevelStyleId},
    paragraph::Paragraph,
};
use image::{DynamicImage, ImageFormat, ImageReader, ImageResult};
use serde::{Deserialize, Serialize};

to_rdocx_static_dispatch! {
    {#[derive(Debug, Clone, Serialize, Deserialize)]},
    <{D: rdocx::Document}>,
    Content {
        Paragraph
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Stile {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub lists: HashMap<ListLevelStyleId, Vec<ListLevel>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub pictures: HashMap<String, PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub contents: Vec<Content>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub stile: Stile,
}

impl Document {
    pub fn to_rdocx(&self) -> (rdocx::Document, Vec<rdocx_decl::Error>) {
        let mut errs = vec![];
        let mut result = rdocx::Document::new();
        let mut cash = Cash {
            lists: self
                .stile
                .lists
                .iter()
                .map(|(name, list)| {
                    (
                        name.clone(),
                        result.add_list_definition(
                            list.iter()
                                .cloned()
                                .map(Into::into)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                    )
                })
                .collect(),
            pictures: self
                .stile
                .pictures
                .iter()
                .map(|(name, path)| {
                    (name.clone(), {
                        let image = ImageReader::open(path)
                            .map(ImageReader::decode)
                            .map(|image| image.map(dynamic_image_to_png_bytes));
                        match image {
                            Ok(Ok(Ok(image))) => match path.file_name().map(OsStr::to_string_lossy)
                            {
                                Some(file_name) => Some(result.embed_image(&image, &file_name)),
                                None => {
                                    errs.push(rdocx_decl::Error::UndefinedFileName {
                                        path: path.clone(),
                                    });
                                    Some(result.embed_image(
                                        &image,
                                        &format!("Error: undefined name in file path {:?}", path),
                                    ))
                                }
                            },
                            Ok(Ok(Err(err))) => {
                                errs.push(rdocx_decl::Error::ImageEncode(err));
                                None
                            }
                            Ok(Err(err)) => {
                                errs.push(rdocx_decl::Error::ImageDecode(err));
                                None
                            }
                            Err(err) => {
                                errs.push(rdocx_decl::Error::ImageOpen(err));
                                None
                            }
                        }
                    })
                })
                .collect(),
        };

        errs.extend(
            self.contents
                .iter()
                .flat_map(|content| content.to_rdocx(&mut result, (), &mut cash))
                .collect::<Vec<_>>(),
        );

        (result, errs)
    }
}

fn dynamic_image_to_png_bytes(img: DynamicImage) -> ImageResult<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
    Ok(bytes)
}
