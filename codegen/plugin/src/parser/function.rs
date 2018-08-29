use syntax::ast::*;
use syntax::codemap::{Span, Spanned};
use syntax::ext::base::Annotatable;
use utils;

#[derive(Debug)]
pub struct Function(Spanned<(Ident, FnDecl)>);

impl Function {
    pub fn from(annotated: &Annotatable) -> Result<Function, Span> {
        if let Annotatable::Item(ref item) = *annotated {
            if let ItemKind::Fn(ref decl, ..) = item.node {
                let inner = (item.ident, decl.clone().into_inner());
                return Ok(Function(utils::span(inner, item.span)));
            }
        }

        Err(annotated.span())
    }

    pub fn ident(&self) -> &Ident {
        &self.0.node.0
    }

    pub fn decl(&self) -> &FnDecl {
        &self.0.node.1
    }

    pub fn span(&self) -> Span {
        self.0.span
    }
}
