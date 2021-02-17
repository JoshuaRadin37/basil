#[macro_use]
extern crate basil_derive;

pub mod context;
pub mod frame;
pub mod interpreter;
pub mod reference_chain;

#[cfg(test)]
mod tests {
    use super::*;
    use basil_core::span::WithSpan;
    use std::path::PathBuf;

    #[test]
    fn create_span() {
        let line1 = line!();
        let with_span: WithSpan<_> = span!(14 + 3);
        let line2 = line!();
        println!("{:?}", with_span.get_span());
        let span = with_span.get_span();
        assert_eq!(with_span.get_span().file(), &PathBuf::from(file!()));
        assert!(with_span.get_span().start().line > line1 as usize);
        assert!(with_span.get_span().end().line < line2 as usize);
    }
}
