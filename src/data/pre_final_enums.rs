#[crabtime::function]
fn pre_final_enums(
    pattern!(
        $name1:ident,
        $name2:ident,
        {
            $(
                $variant:ident
                $( ( $($type:ty),* $(,)? ) )?
                $( { $($field:ident: $ftype:ty),* $(,)? } )?
            ),*
            $(,)?
        },
        $into_trait:ident<$err:ty>::$into_fn:ident$(<$into_life:lifetime>)?($($into_field:ident: $into_type:ty),*),
        {$(
            $add_variant:ident
            $( ( $($add_name_for_type:ident: $add_type:ty),* ) )?
            $( { $($add_field:ident: $add_ftype:ty),* $(,)? } )?
            =>
            $into:block
        ),*$(,)?}
    ): _,
) {
    use std::collections::HashMap;
    struct Param {
        name: String,
        list: Vec<(Option<String>, String)>,
        map: HashMap<String, String>,
    }

    impl Param {
        fn to_string_for_enum(&self) -> String {
            [
                self.name.clone(),
                if self.list.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        "({})",
                        self.list
                            .iter()
                            .map(|(_, a)| a)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                },
                if self.map.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        "{{{}}}",
                        self.map
                            .iter()
                            .map(|(a, b)| format!("{a}: {b}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                },
            ]
            .concat()
        }

        fn declaration_param(&self, enum_name: String, into_fn: Option<String>) -> String {
            let name = self.name.clone();
            let list_str = if self.list.is_empty() {
                "".to_string()
            } else {
                format!(
                    "({})",
                    self.list
                        .iter()
                        .enumerate()
                        .map(|(i, (name, ty))| {
                            let name = if let Some(name) = name {
                                name.clone()
                            } else {
                                format!("a{i}")
                            };
                            if let Some(into_fn) = into_fn.clone()
                                && ty.find("Self").is_some()
                            {
                                format!("{{let (result, err) = {name}.{into_fn}?; result_err.extend(err); result}}")
                            } else {
                                name
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            let map_str = if self.map.is_empty() {
                "".to_string()
            } else {
                format!(
                    "{{{}}}",
                    self.map
                        .iter()
                        .map(|(name, ty)| {
                            if let Some(into_fn) = into_fn.clone()
                                && ty.find(" Self ").is_some()
                            {
                                format!("{name}: {{let (result, err) = {name}.{into_fn}?; result_err.extend(err); result}}")
                            } else {
                                name.clone()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };

            crabtime::quote! {
                {{enum_name}}::{{name}}{{list_str}}{{map_str}}
            }
        }

        fn match_param(&self, enum1: String, enum2: String, into_fn: String) -> String {
            let declaration_param1 = self.declaration_param(enum1, Some(into_fn));
            let declaration_param2 = self.declaration_param(enum2, None);

            crabtime::quote! {
                {{declaration_param2}} => {
                    let mut result_err = vec![];
                    let result = {{declaration_param1}};
                    Ok((result, result_err))
                },
            }
        }
    }

    let name1 = stringify!($name1);
    let name2 = stringify!($name2);
    let into_trait = stringify!($into_trait);
    let into_fn = stringify!($into_fn);
    let err = stringify!($err);
    let into_life = expand!([$(stringify!(<$into_life>).to_string())?])
        .first()
        .cloned()
        .unwrap_or("".to_string());
    let base_params = expand!([
        $(Param {
            name: stringify!($variant).to_string(),
            list: [$($((
                None,
                stringify!($type).to_string()
            )),*)?].into_iter().collect(),
            map: {
                let mut result = HashMap::<String, String>::new();
                $($(result.insert(
                    stringify!($field).to_string(),
                    stringify!($ftype).to_string(),
                );)*)?
                result
            }
        }),*
    ])
    .into_iter()
    .collect::<Vec<Param>>();

    let add_params = expand!([
        $((
            Param {
                name: stringify!($add_variant).to_string(),
                list: [
                    $($((
                        Some(stringify!($add_name_for_type).to_string()),
                        stringify!($add_type).to_string(),
                    )),*)?
                ].into_iter().collect(),
                map: {
                    let mut result = HashMap::<String, String>::new();
                    $($(result.insert(
                        stringify!($add_field).to_string(),
                        stringify!($add_ftype).to_string(),
                    );)*)?
                    result
                }
            },
            stringify!($into).to_string()
        )),*
    ])
    .into_iter()
    .collect::<Vec<(Param, String)>>();

    let into_args = expand!({
        let mut result = HashMap::<String, String>::new();
        $(result.insert(
            stringify!($into_field).to_string(),
            stringify!($into_type).to_string(),
        );)*
        result
    });

    let into_args_str = into_args
        .iter()
        .map(|(a, b)| format!("{a}: {b}"))
        .collect::<Vec<_>>()
        .join(", ");

    let into_arg_names_str = into_args.keys().cloned().collect::<Vec<_>>().join(", ");

    let into_dec = crabtime::quote!({ { into_fn } }{{into_life}}(self, { { into_args_str } }));
    let into_run = crabtime::quote!({ { into_fn } }({ { into_arg_names_str } }));

    let into_args_cloned_list_str = into_args
        .iter()
        .map(|(a, b)| format!("{a}.clone()"))
        .collect::<Vec<_>>()
        .join(", ");

    let base_params_str = base_params
        .iter()
        .map(|param| param.to_string_for_enum())
        .collect::<Vec<_>>()
        .join(", ");
    let add_params_str = add_params
        .iter()
        .map(|(param, _)| param.to_string_for_enum())
        .collect::<Vec<_>>()
        .join(", ");

    let base_match = base_params
        .iter()
        .map(|param| param.match_param(name1.to_string(), name2.to_string(), into_run.clone()))
        .collect::<Vec<_>>()
        .concat();
    let add_match = add_params
        .iter()
        .map(|(param, into)| {
            let declaration_param = param.declaration_param(name2.to_string(), None);
            crabtime::quote! {
                {{declaration_param}} => {{into}},
            }
        })
        .collect::<Vec<_>>()
        .concat();

    crabtime::output! {
        pub trait {{into_trait}}<T, E> {
            fn {{into_dec}} -> Result<(T, Vec<E>), Vec<E>>;
        }

        impl<E, T2, T1: {{into_trait}}<T2, E>> {{into_trait}}<Vec<T2>, E> for Vec<T1> {
            fn {{into_dec}} -> Result<(Vec<T2>, Vec<E>), Vec<E>> {
                let results = self
                    .into_iter()
                    .map(|data| data.{{into_fn}}({{into_args_cloned_list_str}}))
                    .collect::<Vec<_>>();

                if results.iter().any(|result|result.is_err()) {
                    Err(results.into_iter().flat_map(|result| match result {
                        Ok((_, err)) => err,
                        Err(err) => err,
                    }).collect())
                } else {
                    let mut result_data = vec![];
                    let mut result_err = vec![];

                    results.into_iter().for_each(|result| match result {
                        Ok((data, err)) => {
                            result_data.push(data);
                            result_err.extend(err);
                        }
                        Err(err) => unreachable!()
                    });
                    Ok((result_data, result_err))
                }
            }
        }

        impl<E, T2, T1: {{into_trait}}<T2, E>> {{into_trait}}<Box<T2>, E> for Box<T1> {
            fn {{into_dec}} -> Result<(Box<T2>, Vec<E>), Vec<E>> {
                (*self).{{into_fn}}({{into_args_cloned_list_str}}).map(|(data, err)| (Box::new(data), err))
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum {{name1}}{ {{base_params_str}} }
        pub enum {{name2}}{ {{base_params_str}}, {{add_params_str}} }

        impl {{into_trait}}<{{name1}}, {{err}}> for {{name2}} {
            fn {{into_dec}} -> Result<({{name1}}, Vec<{{err}}>), Vec<{{err}}>> {
                match self {
                    {{base_match}}
                    {{add_match}}
                }
            }
        }
    }
}
