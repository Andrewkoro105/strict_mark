#[crabtime::function]
fn to_rdocx_static_dispatch(
    pattern!(
        $({$($macro:tt)*},)?
        $(impl{$($generic0:tt)*},)?
        <{D: $($generic1:tt)*} $(, {T: $($generic2:tt)*})?>,
        $name:ident
        {
            $(
                $elements:ident
            ),*
            $(,)?
        }
    ): _,
) {
    let mut result = String::new();

    let drive = stringify!($($($macro)*)?);
    let generic0 = stringify!($($($generic0)*)?);
    let generic1 = stringify!($($generic1)*);
    let mut generic2 = stringify!($($($generic2)*)?);
    if generic2.is_empty() {
        generic2 = "()";
    }

    let name = stringify!($name);
    let elements = expand!([$(stringify!($elements).to_string()),*])
        .into_iter()
        .collect::<Vec<String>>();
    //result.push_str(&format!(crabtime::quote! {use crate::rdocx_decl::ToRdocx;};
    //result.push_str(&format!("use crate::rdocx_decl::ToRdocx;\n");
    result.push_str(&format!("{drive} pub enum {name} {{\n"));
    elements.iter().for_each(|element| {
        let _ = 1;
        result.push_str(&format!("{element}({element}),\n"));
    });
    result.push_str(&format!("}}\n"));

    elements.iter().for_each(|element| {
        result.push_str(&format!("{}", crabtime::quote! {
            impl From<{{element}}> for {{name}} {
                fn from(val: {{element}}) -> Self {
                    Self::{{element}}(val)
                }
            }
        }))
    });

    result.push_str(&format!(
        r"
        impl<{generic0}> ToRdocx<{generic1}, {generic2}> for {name} {{
            fn to_rdocx(&self, doc: &mut {generic1}, data: {generic2}, cash: &mut Cash) {{
                match self {{
    "
    ));

    elements.iter().for_each(|element| {
        result.push_str(&format!("{}", crabtime::quote! {
            Self::{{element}}(inner) => inner.to_rdocx(doc, data, cash),
        }))
    });
    result.push_str(&format!(
        r"
                }}
            }}
        }}
    "
    ));
    //print!("{}", result);
    crabtime::output_str!("{}", result);
}
