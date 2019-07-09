use proc_macro::{Diagnostic, Literal, Span};
use std::ops::{Deref, RangeBounds};

pub type PResult<T> = ::std::result::Result<T, Diagnostic>;

// An experiment.
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics(vec![])
    }

    pub fn push(&mut self, diag: Diagnostic) {
        self.0.push(diag);
    }

    pub fn emit_head(self) -> Diagnostic {
        let mut iter = self.0.into_iter();
        let mut last = iter.next().expect("Diagnostic::emit_head empty");
        for diag in iter {
            last.emit();
            last = diag;
        }

        last
    }

    pub fn head_err_or<T>(self, ok: T) -> PResult<T> {
        if self.0.is_empty() {
            Ok(ok)
        } else {
            Err(self.emit_head())
        }
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(diag: Diagnostic) -> Self {
        Diagnostics(vec![diag])
    }
}

impl From<Vec<Diagnostic>> for Diagnostics {
    fn from(diags: Vec<Diagnostic>) -> Self {
        Diagnostics(diags)
    }
}

pub struct StringLit(crate String, crate Literal);

impl Deref for StringLit {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl StringLit {
    pub fn new<S: Into<String>>(string: S, span: Span) -> Self {
        let string = string.into();
        let mut lit = Literal::string(&string);
        lit.set_span(span);
        StringLit(string, lit)
    }

    pub fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {
        self.1.subspan(range)
    }
}
