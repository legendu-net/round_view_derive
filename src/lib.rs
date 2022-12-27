use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput};

#[proc_macro_derive(R2View)]
pub fn r2_view_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline(always)]
            fn r2_view(&mut self) -> RoundTwoView {
                RoundTwoView {
                    rng_combs_r1: &self.rng_combs_r1,
                    rng_combs_r2: &mut self.rng_combs_r2,
                    rng_combs_r3: &mut self.rng_combs_r3,
                    rng_combs_r4: &mut self.rng_combs_r4,
                    play: &mut self.play,
                    cache_16: &mut self.cache_16,
                    cache_21: &mut self.cache_21,
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(R3View)]
pub fn r3_view_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline(always)]
            fn r3_view(&mut self) -> RoundThreeView {
                RoundThreeView {
                    rng_combs_r2: &self.rng_combs_r2,
                    rng_combs_r3: &mut self.rng_combs_r3,
                    rng_combs_r4: &mut self.rng_combs_r4,
                    play: &mut self.play,
                    cache_16: &mut self.cache_16,
                    cache_21: &mut self.cache_21,
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(R4View)]
pub fn r4_view_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline(always)]
            fn r4_view(&mut self) -> RoundFourView {
                RoundFourView {
                    rng_combs_r3: &self.rng_combs_r3,
                    rng_combs_r4: &mut self.rng_combs_r4,
                    play: &mut self.play,
                    cache_16: &mut self.cache_16,
                    cache_21: &mut self.cache_21,
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(R5View)]
pub fn r5_view_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline(always)]
            fn r5_view(&mut self) -> RoundFiveView {
                RoundFiveView {
                    rng_combs_r4: &self.rng_combs_r4,
                    play: &mut self.play,
                }
            }
        }
    };
    gen.into()
}
