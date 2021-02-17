#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::parse_macro_input::parse;
use syn::spanned::Spanned;
use syn::Expr;

use basil_core::span::LineColumn as MyLineColumn;
use basil_core::span::Span;
use basil_core::span::WithSpan;

/// Creates a [WithSpan] object here, with this current file and line as the span
///
/// [WithSpan]: basil_core::span::WithSpan
/*
#[macro_export]
macro_rules! span {
    ($inner:expr) => { span!(@ column!(), $inner, line!())};
    (@ $c1:expr, $inner:expr, $line:expr ) => {{
        let stringified = stringify!(span!($inner));
        println!("{}", stringified);
        let len = stringified.len();
        let c2 = $c1 + len as u32;
        let range = ($c1 as u64)..(c2 as u64);
        let file = file!();
        println!("{}", file);
        let span = basil_core::span::Span::new(std::path::PathBuf::from(file), $line, range);
        basil_core::span::WithSpan::new($inner, span)
    }};
}

 */

#[proc_macro]
pub fn span(stream: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(stream as Expr);
    let start = parsed.span().start();
    let end = parsed.span().end();

    let start_line = start.line;
    let start_col = start.column;

    let end_line = end.line;
    let end_col = end.column;

    let tokens = quote! {
        {

            let start = basil_core::span::LineColumn::new(#start_line, #start_col);
            let end = basil_core::span::LineColumn::new(#end_line, #end_col);
            let canonical = std::path::PathBuf::from(file!());
            let span = basil_core::span::Span::new(canonical, start, end);
            basil_core::span::WithSpan::new(#parsed, span)
        }
    };

    TokenStream::from(tokens)
}
