use std::collections::HashMap;

use super::is_default;
use crate::rdocx_decl::{
    Cash, ToRdocx,
    list_level::{ListLevel, ListLevelStyleId},
    paragraph::Paragraph,
};
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
    lists: HashMap<ListLevelStyleId, Vec<ListLevel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub contents: Vec<Content>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub stile: Stile,
}

impl Document {
    pub fn to_rdocx(&self) -> rdocx::Document {
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
        };

        self.contents
            .iter()
            .for_each(|content| content.to_rdocx(&mut result, (), &mut cash));

        result
    }
}
