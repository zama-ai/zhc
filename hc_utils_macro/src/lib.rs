use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Variant, parse_macro_input};

#[proc_macro_attribute]
pub fn fsm(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Check that it's applied to an enum
    let Data::Enum(mut enum_data) = input.data else {
        return syn::Error::new_spanned(input, "fsm can only be applied to enums")
            .to_compile_error()
            .into();
    };

    // Add __INVALID variant
    let invalid_variant = Variant {
        attrs: vec![],
        ident: Ident::new("__INVALID", proc_macro2::Span::call_site()),
        fields: Fields::Unit,
        discriminant: None,
    };
    enum_data.variants.push(invalid_variant);

    let enum_name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let variants = enum_data.variants.iter();

    let expanded = quote! {
        #(#attrs)*
        #vis enum #enum_name #generics {
            #(#variants,)*
        }

        impl #impl_generics #enum_name #ty_generics #where_clause {
            /// Transitions the FSM state using the provided function.
            ///
            /// The function receives the current state and must return the new state.
            /// This method safely handles the transition by temporarily setting the
            /// state to __INVALID during the transformation.
            pub fn transition<F>(&mut self, mut transitioner: F)
            where
                F: FnOnce(Self) -> Self
            {
                let old_state = std::mem::replace(self, Self::__INVALID);
                *self = transitioner(old_state);
            }
        }
    };

    expanded.into()
}
