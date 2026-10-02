use crate::rdocx_decl::{Cash, ToRdocx, paragraph::Paragraph};
use serde::{Deserialize, Serialize};

to_rdocx_static_dispatch!{
    {#[derive(Debug, Clone, Serialize, Deserialize)]},
    <{D: rdocx::Document}>,
    Content {
        Paragraph
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stile {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub contents: Vec<Content>,
}

impl Document {
    pub fn to_rdocx(&self) -> rdocx::Document {
        let mut result = rdocx::Document::new();
        let mut cash = Cash::default();
        self.contents
            .iter()
            .for_each(|content| content.to_rdocx(&mut result, (), &mut cash));
        result
    }
}