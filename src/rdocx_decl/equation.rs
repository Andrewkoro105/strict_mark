use rdocx::{self, CT_OMath, OfficeMath, equation_from_latex};
use serde::{Deserialize, Serialize};

use crate::rdocx_decl::{ToRdocx, run};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquationType {
    Inline,
    Display,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub latex: String,
    pub equation_type: EquationType
}

impl<'p> ToRdocx<rdocx::Paragraph<'p>, &Option<run::Style>> for Equation {
    fn to_rdocx(&self, paragraph: &mut rdocx::Paragraph<'p>, _data: &Option<run::Style>, _cash: &mut super::Cash) {

        //todo: Add error handling.
        let conversion_result = equation_from_latex(&self.latex).unwrap();

        //todo: Add error handling.
        let _ = paragraph.add_equation(match self.equation_type {
            EquationType::Inline => OfficeMath::inline(conversion_result.value.expressions),
            EquationType::Display => OfficeMath::display(vec![CT_OMath::new(conversion_result.value.expressions)]),
        });
    }
}