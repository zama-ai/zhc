use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, FnArg, Ident, ItemFn, Pat, Token, bracketed,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

mod presets {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn all_iops() -> TokenStream {
        quote! {
            [
                cmp_gt = Iop::CmpGt,
                cmp_gte = Iop::CmpGte,
                cmp_lt = Iop::CmpLt,
                cmp_lte = Iop::CmpLte,
                cmp_eq = Iop::CmpEq,
                cmp_neq = Iop::CmpNeq,
                if_then_else = Iop::IfThenElse,
                if_then_zero = Iop::IfThenZero,
                add = Iop::Add,
                add_simd = Iop::AddSimd,
                sub = Iop::Sub,
                mul = Iop::Mul,
                muls = Iop::Muls,
                ilog2 = Iop::Ilog2,
                count_zeros = Iop::CountZeros,
                count_ones = Iop::CountOnes,
                leading_zeros = Iop::LeadingZeros,
                leading_ones = Iop::LeadingOnes,
                trailing_zeros = Iop::TrailingZeros,
                trailing_ones = Iop::TrailingOnes,
                div = Iop::Div,
                r#mod = Iop::Mod,
                divs = Iop::Divs,
                mods = Iop::Mods,
                adds = Iop::Adds,
                subs = Iop::Subs,
                ssub = Iop::Ssub,
                ovf_adds = Iop::OvfAdds,
                ovf_muls = Iop::OvfMuls,
                ovf_subs = Iop::OvfSubs,
                ovf_ssub = Iop::OvfSsub,
                bw_and = Iop::BwAnd,
                bw_or = Iop::BwOr,
                bw_xor = Iop::BwXor,
                bw_not = Iop::BwNot,
                right_shift = Iop::RightShift,
                left_shift = Iop::LeftShift,
                right_rot = Iop::RightRot,
                left_rot = Iop::LeftRot,
                right_shifts = Iop::RightShifts,
                left_shifts = Iop::LeftShifts,
                right_rots = Iop::RightRots,
                left_rots = Iop::LeftRots,
                ovf_add = Iop::OvfAdd,
                ovf_sub = Iop::OvfSub,
                ovf_mul = Iop::OvfMul,
                erc7984 = Iop::Erc7984,
                erc7984_simd = Iop::Erc7984Simd,
                mem_cpy = Iop::MemCpy,
                cast_2 = Iop::Cast { to_size: 2 },
                cast_4 = Iop::Cast { to_size: 4 },
                cast_8 = Iop::Cast { to_size: 8 },
                cast_16 = Iop::Cast { to_size: 16 },
                cast_32 = Iop::Cast { to_size: 32 },
                cast_64 = Iop::Cast { to_size: 64 },
                cast_128 = Iop::Cast { to_size: 128 },
                flip = Iop::Flip,
                sum_3 = Iop::Sum { n: 3 },
                sum_5 = Iop::Sum { n: 5 },
                sum_6 = Iop::Sum { n: 6 },
                sum_9 = Iop::Sum { n: 9 },
                sum_11 = Iop::Sum { n: 11 },
                sum_13 = Iop::Sum { n: 13 },
                sum_17 = Iop::Sum { n: 17 },
                sum_26 = Iop::Sum { n: 26 },
            ]
        }
    }

    pub fn all_precs() -> TokenStream {
        quote! {
            [
                s2 = 2, s4 = 4, s6 = 6, s8 = 8, s10 = 10, s12 = 12, s14 = 14, s16 = 16,
                s18 = 18, s20 = 20, s22 = 22, s24 = 24, s26 = 26, s28 = 28, s30 = 30, s32 = 32,
                s34 = 34, s36 = 36, s38 = 38, s40 = 40, s42 = 42, s44 = 44, s46 = 46, s48 = 48,
                s50 = 50, s52 = 52, s54 = 54, s56 = 56, s58 = 58, s60 = 60, s62 = 62, s64 = 64,
                s66 = 66, s68 = 68, s70 = 70, s72 = 72, s74 = 74, s76 = 76, s78 = 78, s80 = 80,
                s82 = 82, s84 = 84, s86 = 86, s88 = 88, s90 = 90, s92 = 92, s94 = 94, s96 = 96,
                s98 = 98, s100 = 100, s102 = 102, s104 = 104, s106 = 106, s108 = 108, s110 = 110,
                s112 = 112, s114 = 114, s116 = 116, s118 = 118, s120 = 120, s122 = 122,
                s124 = 124, s126 = 126, s128 = 128,
            ]
        }
    }
}

struct Case {
    name: Ident,
    value: Expr,
}

impl Parse for Case {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Case { name, value })
    }
}

struct AxisArg {
    param: Ident,
    cases: Vec<Case>,
}

impl Parse for AxisArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let param = input.parse()?;
        input.parse::<Token![=]>()?;
        let cases = input.parse::<Axis>()?.0;
        Ok(AxisArg { param, cases })
    }
}

struct Axis(Vec<Case>);

impl Parse for Axis {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let cases = Punctuated::<Case, Token![,]>::parse_terminated(&content)?;
            if cases.is_empty() {
                return Err(input.error("axis is empty"));
            }
            return Ok(Axis(cases.into_iter().collect()));
        }
        if lookahead.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            let name: Ident = input.parse()?;
            let tokens = match name.to_string().as_str() {
                "all_iops" => presets::all_iops(),
                "all_precs" => presets::all_precs(),
                other => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("unknown preset `@{other}`"),
                    ));
                }
            };
            return syn::parse2::<Axis>(tokens);
        }
        Err(lookahead.error())
    }
}

fn axes_in_param_order(args: &[AxisArg], func: &ItemFn) -> syn::Result<Vec<Vec<Case>>> {
    let mut axes = Vec::with_capacity(func.sig.inputs.len());
    for input in &func.sig.inputs {
        let FnArg::Typed(pat_ty) = input else {
            return Err(syn::Error::new(input.span(), "`self` is not supported"));
        };
        let Pat::Ident(pat) = &*pat_ty.pat else {
            return Err(syn::Error::new(
                pat_ty.pat.span(),
                "parameter must be a plain identifier",
            ));
        };
        let Some(arg) = args.iter().find(|a| a.param == pat.ident) else {
            return Err(syn::Error::new(
                pat.ident.span(),
                format!("no axis given for parameter `{}`", pat.ident),
            ));
        };
        axes.push(
            arg.cases
                .iter()
                .map(|c| Case {
                    name: c.name.clone(),
                    value: c.value.clone(),
                })
                .collect(),
        );
    }
    for arg in args {
        let is_param = func.sig.inputs.iter().any(|input| {
            matches!(input, FnArg::Typed(t) if matches!(&*t.pat, Pat::Ident(p) if p.ident == arg.param))
        });
        if !is_param {
            return Err(syn::Error::new(
                arg.param.span(),
                format!("axis `{}` matches no function parameter", arg.param),
            ));
        }
    }
    if axes.is_empty() {
        return Err(syn::Error::new(
            func.sig.span(),
            "function has no parameter",
        ));
    }
    Ok(axes)
}

fn expand(args: Vec<AxisArg>, mut func: ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let axes = axes_in_param_order(&args, &func)?;

    let (doc_attrs, test_attrs): (Vec<_>, Vec<_>) =
        func.attrs.drain(..).partition(|a| a.path().is_ident("doc"));
    func.attrs = doc_attrs;
    func.attrs.push(syn::parse_quote!(#[allow(dead_code)]));

    let fn_name = &func.sig.ident;
    let mut tests = Vec::new();
    let mut idx = vec![0usize; axes.len()];
    loop {
        let mut name = fn_name.to_string();
        let mut call_args = Vec::with_capacity(axes.len());
        for (k, axis) in axes.iter().enumerate() {
            let case = &axis[idx[k]];
            name.push('_');
            name.push_str(&case.name.unraw().to_string());
            call_args.push(&case.value);
        }
        let test = format_ident!("{name}", span = fn_name.span());
        tests.push(quote! {
            #[test]
            #(#test_attrs)*
            fn #test() { #fn_name(#(#call_args),*) }
        });

        // Odometer increment over the axes.
        let mut k = axes.len();
        loop {
            if k == 0 {
                return Ok(quote! {
                    #func
                    #(#tests)*
                });
            }
            k -= 1;
            idx[k] += 1;
            if idx[k] < axes[k].len() {
                break;
            }
            idx[k] = 0;
        }
    }
}

/// See the crate documentation.
#[proc_macro_attribute]
pub fn test_matrix(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<AxisArg, Token![,]>::parse_terminated);
    let func = parse_macro_input!(item as ItemFn);
    match expand(args.into_iter().collect(), func) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
