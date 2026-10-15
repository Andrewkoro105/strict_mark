use rdocx::{self, CT_OMath, OfficeMath, equation_from_latex};
use serde::{Deserialize, Serialize};

use crate::rdocx_decl::{self, ToRdocx, run};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquationType {
    Inline,
    Display,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub latex: String,
    pub equation_type: EquationType,
}

impl<'p> ToRdocx<rdocx::Paragraph<'p>, &Option<run::Style>> for Equation {
    fn to_rdocx(
        &self,
        paragraph: &mut rdocx::Paragraph<'p>,
        _data: &Option<run::Style>,
        _cash: &mut super::Cash,
    ) -> Vec<rdocx_decl::Error> {
        let mut err = vec![];
        let conversion_result = equation_from_latex(&self.latex).unwrap();
        err.push(rdocx_decl::Error::RdocxMathConversion(
            conversion_result.diagnostics,
        ));

        let add_equation_result = paragraph.add_equation(match self.equation_type {
            EquationType::Inline => OfficeMath::inline(conversion_result.value.expressions),
            EquationType::Display => {
                OfficeMath::display(vec![CT_OMath::new(conversion_result.value.expressions)])
            }
        });

        if let Err(add_equation_err) = add_equation_result {
            err.push(rdocx_decl::Error::RdocxAddEquation(add_equation_err));
        }

        err
    }
}
