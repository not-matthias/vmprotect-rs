use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, Stmt};

/// Creates the marker by calling the correct function from the vmprotect sdk.
fn start_marker(prot_type: &str, fn_name: &str) -> syn::Result<Stmt> {
    let token_stream = match prot_type {
        "mutation" => {
            quote! {
                unsafe {
                   vmprotect_sys::VMProtectBeginMutation(vmprotect_sys::real_c_string::real_c_string!(#fn_name))
                }
            }
        }
        "virtualization" => {
            quote! {
                unsafe {
                  vmprotect_sys::VMProtectBeginVirtualization(vmprotect_sys::real_c_string::real_c_string!(#fn_name))
                }
            }
        }
        "ultra" => {
            quote! {
                unsafe {
                    vmprotect_sys::VMProtectBeginUltra(vmprotect_sys::real_c_string::real_c_string!(#fn_name))
                }
            }
        }
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Invalid marker type",
            ))
        }
    };

    syn::parse(TokenStream::from(token_stream))
}

/// Creates the end marker by calling the `VMProtectEnd` function from the
/// vmprotect sdk.
fn end_marker() -> syn::Result<Stmt> {
    // If you have a `loop` that does not exit, rust will complain that the marker
    // is not reachable. However, since the marker should not be executed, this does
    // not matter.
    //
    let token_stream = quote! {
        unsafe { vmprotect_sys::VMProtectEnd() };
    };

    syn::parse(TokenStream::from(token_stream))
}

#[proc_macro_attribute]
pub fn vmprotect(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut original_fn: ItemFn = parse_macro_input!(input as ItemFn);

    let mut attr = attr.into_iter();
    let prot_type = if let Some(prot_type @ TokenTree::Ident { .. }) = attr.next() {
        prot_type.to_string().to_lowercase()
    } else {
        panic!("missing protection type")
    };

    if !["mutate", "virtualize", "ultra"].contains(&prot_type.as_ref()) {
        panic!("unknown protection type: {:?}", prot_type);
    }

    let name = match attr.next() {
        Some(TokenTree::Punct(..)) => match attr.next() {
            Some(TokenTree::Literal(lit)) => lit.to_string(),
            Some(_) => panic!("expected name"),
            None => panic!("expected name"),
        },
        Some(_) => panic!("expected name"),
        None => "unknown".to_string(),
    };

    let start = match start_marker(&prot_type, name.as_str()) {
        Ok(start) => start.to_token_stream(),
        Err(e) => return e.to_compile_error().into(),
    };

    let end = match end_marker() {
        Ok(end) => end.to_token_stream(),
        Err(e) => return e.to_compile_error().into(),
    };

    // Create the new function block
    //
    let original_block = original_fn.block.into_token_stream();
    let block = quote! {
        {
            #[allow(unreachable_code)]
            {
                #start
                unsafe { core::arch::asm!("nop") };
                let return_value = core::hint::black_box(#original_block);
                unsafe { core::arch::asm!("nop") };
                #end
                return_value
            }
        }
    };

    // Parse the block
    //
    let block = match syn::parse2(block) {
        Ok(block) => block,
        Err(e) => return e.to_compile_error().into(),
    };

    // Set the new function block
    //
    original_fn.block = block;

    original_fn.into_token_stream().into()
}
