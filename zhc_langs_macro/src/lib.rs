use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{Expr, Ident, ItemFn, Local, Pat, Stmt, Type, parse_macro_input, spanned::Spanned};

struct Value {
    name: Ident,
    ty: Ident,
}

struct Step {
    func: Ident,
    variant: Ident,
    affinity: Ident,
    args: Vec<Ident>,
    rets: Vec<Ident>,
}

struct Program {
    affinities: Vec<Ident>,
    values: Vec<Value>,
    steps: Vec<Step>,
}

#[proc_macro_attribute]
pub fn gen_pipeline_lang(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(input: &ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let program = parse_program(input)?;
    let prefix = to_pascal_case(&input.sig.ident.to_string());
    let span = input.sig.ident.span();
    let vis = &input.vis;

    let lang = format_ident!("{prefix}Lang", span = span);
    let type_system = format_ident!("{prefix}TypeSystem", span = span);
    let instruction_set = format_ident!("{prefix}InstructionSet", span = span);
    let affinity = format_ident!("{prefix}Affinity", span = span);
    let val_ids = format_ident!("{prefix}ValIds", span = span);
    let static_name = format_ident!(
        "__{}_LANG",
        input.sig.ident.to_string().to_uppercase(),
        span = span
    );

    let types: HashMap<String, &Ident> = program
        .values
        .iter()
        .map(|v| (v.name.to_string(), &v.ty))
        .collect();

    let mut type_variants: Vec<&Ident> = Vec::new();
    for (index, value) in program.values.iter().enumerate() {
        if let Some(previous) = program.values[..index]
            .iter()
            .find(|other| other.ty == value.ty)
        {
            return Err(syn::Error::new(
                value.ty.span(),
                format!(
                    "type `{}` is already produced by `{}`; every value needs its own type",
                    value.ty, previous.name
                ),
            ));
        }
        type_variants.push(&value.ty);
    }

    let affinity_variants = &program.affinities;

    let mut instruction_variants: Vec<&Ident> = Vec::new();
    let mut affinity_arms = Vec::new();
    let mut format_arms = Vec::new();
    let mut signature_arms = Vec::new();

    for step in &program.steps {
        let variant = &step.variant;
        let arg_types: Vec<&Ident> = step.args.iter().map(|a| types[&a.to_string()]).collect();
        let ret_types: Vec<&Ident> = step.rets.iter().map(|r| types[&r.to_string()]).collect();
        if instruction_variants.contains(&variant) {
            return Err(syn::Error::new(
                step.func.span(),
                format!("step `{}` is already defined", step.func),
            ));
        }

        instruction_variants.push(variant);
        let step_affinity = &step.affinity;
        affinity_arms.push(quote! { Self::#variant => #affinity::#step_affinity });
        let display = step.func.to_string();
        format_arms.push(quote! { Self::#variant => f.write_str(#display) });
        signature_arms.push(quote! {
            Self::#variant => ::zhc_ir::Signature(
                ::zhc_utils::svec![#(#type_system::#arg_types),*],
                ::zhc_utils::svec![#(#type_system::#ret_types),*],
            )
        });
    }

    let value_names: Vec<&Ident> = program.values.iter().map(|v| &v.name).collect();

    let build_steps = program.steps.iter().map(|step| {
        let variant = &step.variant;
        let args = &step.args;
        let binds = step.rets.iter().enumerate().map(|(i, ret)| {
            quote! { let #ret = rets[#i]; }
        });
        quote! {
            let (_, rets) = ir.add_op(#instruction_set::#variant, ::zhc_utils::svec![#(#args),*]);
            #(#binds)*
        }
    });

    Ok(quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #vis enum #affinity {
            #(#affinity_variants,)*
        }

        #[derive(::zhc_utils::DisplayVariant, Debug, Clone, PartialEq, Eq, Hash)]
        #vis enum #type_system {
            #(#type_variants,)*
        }

        impl ::zhc_ir::DialectTypeSystem for #type_system {}

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #vis enum #instruction_set {
            #(#instruction_variants,)*
        }

        impl #instruction_set {
            #vis fn get_affinity(&self) -> #affinity {
                match self {
                    #(#affinity_arms,)*
                }
            }
        }

        impl ::zhc_ir::Format for #instruction_set {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
                _ctx: &::zhc_ir::FormatContext,
            ) -> ::std::fmt::Result {
                match self {
                    #(#format_arms,)*
                }
            }
        }

        impl ::std::fmt::Display for #instruction_set {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::zhc_ir::Format::fmt(self, f, &::zhc_ir::FormatContext::default())
            }
        }

        impl ::zhc_ir::DialectInstructionSet for #instruction_set {
            type TypeSystem = #type_system;

            fn get_signature(&self) -> ::zhc_ir::Signature<#type_system> {
                match self {
                    #(#signature_arms,)*
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #vis struct #lang;

        impl ::zhc_ir::Dialect for #lang {
            type TypeSystem = #type_system;
            type InstructionSet = #instruction_set;
        }

        #[derive(Debug, Clone, Copy)]
        #vis struct #val_ids {
            #(#vis #value_names: ::zhc_ir::ValId,)*
        }

        static #static_name: ::std::sync::LazyLock<(::zhc_ir::IR<#lang>, #val_ids)> =
            ::std::sync::LazyLock::new(|| {
                let mut ir = ::zhc_ir::IR::<#lang>::empty();
                #(#build_steps)*
                (ir, #val_ids { #(#value_names,)* })
            });

        impl #lang {
            #vis fn ir() -> &'static ::zhc_ir::IR<#lang> {
                &#static_name.0
            }

            #vis fn val_ids() -> &'static #val_ids {
                &#static_name.1
            }
        }
    })
}

fn parse_program(input: &ItemFn) -> syn::Result<Program> {
    if !input.sig.inputs.is_empty() {
        return Err(syn::Error::new(
            input.sig.inputs.span(),
            "the pipeline function takes no arguments",
        ));
    }

    let mut program = Program {
        affinities: Vec::new(),
        values: Vec::new(),
        steps: Vec::new(),
    };

    for stmt in &input.block.stmts {
        let Stmt::Expr(Expr::Block(block), _) = stmt else {
            return Err(syn::Error::new(
                stmt.span(),
                "expected a labeled block such as `'commons: { ... }`",
            ));
        };
        let Some(label) = &block.label else {
            return Err(syn::Error::new(
                block.span(),
                "every block needs an affinity label such as `'commons:`",
            ));
        };
        let affinity = Ident::new(
            &to_pascal_case(&label.name.ident.to_string()),
            label.name.ident.span(),
        );
        if !program.affinities.contains(&affinity) {
            program.affinities.push(affinity.clone());
        }

        for stmt in &block.block.stmts {
            let Stmt::Local(local) = stmt else {
                return Err(syn::Error::new(
                    stmt.span(),
                    "expected a `let` statement binding the outputs of a step",
                ));
            };
            parse_step(local, &affinity, &mut program)?;
        }
    }

    Ok(program)
}

fn parse_step(local: &Local, affinity: &Ident, program: &mut Program) -> syn::Result<()> {
    let (names, annotations) = parse_pattern(&local.pat)?;

    let Some(init) = &local.init else {
        return Err(syn::Error::new(local.span(), "a step must be initialized"));
    };
    if let Some((_, diverge)) = &init.diverge {
        return Err(syn::Error::new(
            diverge.span(),
            "`else` is not supported here",
        ));
    }
    let Expr::Call(call) = &*init.expr else {
        return Err(syn::Error::new(
            init.expr.span(),
            "a step must be a call such as `check_ioplang(unchecked_ioplang)`",
        ));
    };
    let func = single_ident_expr(&call.func)?;

    let mut args = Vec::new();
    for arg in &call.args {
        let arg = single_ident_expr(arg)?;
        if !program.values.iter().any(|v| v.name == arg) {
            return Err(syn::Error::new(
                arg.span(),
                format!("`{arg}` is not defined by a previous step"),
            ));
        }
        args.push(arg);
    }

    for (name, annotation) in names.iter().zip(annotations) {
        if program.values.iter().any(|v| v.name == *name) {
            return Err(syn::Error::new(
                name.span(),
                format!("`{name}` is already defined"),
            ));
        }
        let ty = match annotation {
            Some(ty) => ty,
            None => Ident::new(&to_pascal_case(&name.to_string()), name.span()),
        };
        program.values.push(Value {
            name: name.clone(),
            ty,
        });
    }

    let variant = Ident::new(&to_pascal_case(&func.to_string()), func.span());
    program.steps.push(Step {
        func,
        variant,
        affinity: affinity.clone(),
        args,
        rets: names,
    });
    Ok(())
}

fn parse_pattern(pat: &Pat) -> syn::Result<(Vec<Ident>, Vec<Option<Ident>>)> {
    match pat {
        Pat::Ident(ident) => Ok((vec![ident.ident.clone()], vec![None])),
        Pat::Tuple(tuple) => {
            let mut names = Vec::new();
            for elem in &tuple.elems {
                let Pat::Ident(ident) = elem else {
                    return Err(syn::Error::new(elem.span(), "expected a plain name"));
                };
                names.push(ident.ident.clone());
            }
            let annotations = vec![None; names.len()];
            Ok((names, annotations))
        }
        Pat::Type(typed) => {
            let (names, _) = parse_pattern(&typed.pat)?;
            let annotations = match &*typed.ty {
                Type::Tuple(tuple) => {
                    if tuple.elems.len() != names.len() {
                        return Err(syn::Error::new(
                            tuple.span(),
                            "the type annotation has a different arity than the pattern",
                        ));
                    }
                    tuple
                        .elems
                        .iter()
                        .map(|ty| single_ident_type(ty).map(Some))
                        .collect::<syn::Result<Vec<_>>>()?
                }
                ty => {
                    if names.len() != 1 {
                        return Err(syn::Error::new(
                            ty.span(),
                            "a tuple pattern needs a tuple type annotation",
                        ));
                    }
                    vec![Some(single_ident_type(ty)?)]
                }
            };
            Ok((names, annotations))
        }
        _ => Err(syn::Error::new(
            pat.span(),
            "expected a name, a tuple of names, or either with a type annotation",
        )),
    }
}

fn single_ident_expr(expr: &Expr) -> syn::Result<Ident> {
    if let Expr::Path(path) = expr
        && let Some(ident) = path.path.get_ident()
    {
        return Ok(ident.clone());
    }
    Err(syn::Error::new(expr.span(), "expected a plain name"))
}

fn single_ident_type(ty: &Type) -> syn::Result<Ident> {
    if let Type::Path(path) = ty
        && let Some(ident) = path.path.get_ident()
    {
        return Ok(ident.clone());
    }
    Err(syn::Error::new(ty.span(), "expected a plain type name"))
}

fn to_pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}
