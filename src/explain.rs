use std::cmp;
use syntax::ast::{Stmt_, Decl_, Pat_, Ty_, Expr_, Block, Mod, Item, Item_, Unsafety, Constness, Visibility, FnDecl, Arg, Ty, Pat, Expr};
use syntax::codemap::{Span, Spanned, NO_EXPANSION, BytePos, Pos};

use tree::ParseNode;

pub trait SpannedExplainParse {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>>;
}

impl SpannedExplainParse for Block {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.explain(), span: self.span }
    }
}

impl SpannedExplainParse for Mod {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.explain(), span: self.inner }
    }
}

impl SpannedExplainParse for Item {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.explain(), span: self.span }
    }
}

impl <T> SpannedExplainParse for Spanned<T> where T: ExplainParse {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.node.explain(), span: self.span }
    }
}

impl SpannedExplainParse for Arg {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.explain(), span: combine_spans(self.ty.span, self.pat.span) }
    }
}

impl SpannedExplainParse for Ty {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.node.explain(), span: self.span }
    }
}

impl SpannedExplainParse for Pat {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.node.explain(), span: self.span }
    }
}

impl SpannedExplainParse for Expr {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.node.explain(), span: self.span }
    }
}

pub trait ExplainParse {
    fn _explain(&self) -> String;
    fn explain(&self) -> ParseNode<String> {
        ParseNode { value: self._explain(), children: self.children() }
    }
    fn children(&self) -> Vec<Spanned<ParseNode<String>>> {
        vec!()
    }
}

impl ExplainParse for Stmt_ {
    fn _explain(&self) -> String {
        match *self {
            Stmt_::StmtDecl(ref decl, _id) => format!("A declaration of {}", decl.node._explain()),
            _ => "some other statement".to_string()
        }
    }
}

impl ExplainParse for Decl_ {
    fn _explain(&self) -> String {
        match *self {
            Decl_::DeclLocal(ref local) => format!("a local assigning {} of type {:?} to {:?}",
                                                local.pat.node._explain(),
                                                local.ty.as_ref().map_or_else(|| "undeclared".to_string(), |ty| ty.node._explain()),
                                                local.init.as_ref().map_or_else(|| "uninitialized".to_string(), |init| init.node._explain())),
            Decl_::DeclItem(ref _item) => "item declaration?".to_string()
        }
    }
}

impl ExplainParse for Pat_ {
    fn _explain(&self) -> String {
        match *self {
            Pat_::PatIdent(_, ref ident, _) => format!("the symbol {}", ident.node.name.as_str()),
            _ => "some other pattern?".to_string()
        }
    }
}

impl ExplainParse for Block {
    fn children(&self) -> Vec<Spanned<ParseNode<String>>> {
        let stmts = self.stmts.iter().map(|x| x.spanned_explain());
        let expr = self.expr.iter().map(|x| x.spanned_explain());
        stmts.chain(expr).collect::<Vec<_>>()
    }
    fn _explain(&self) -> String {
        format!("A block")
    }
}

impl ExplainParse for Mod {
    fn children(&self) -> Vec<Spanned<ParseNode<String>>> {
        self.items.iter().map(|x| x.spanned_explain()).collect::<Vec<_>>()
    }
    fn _explain(&self) -> String {
        "A module declaration".to_string()
    }
}

impl ExplainParse for Ty_ {
    fn _explain(&self) -> String {
        format!("{:?}", self)
    }
}

impl ExplainParse for Expr_ {
    fn _explain(&self) -> String {
        format!("{:?}", self)
    }
}

// An `Item` has the span, name, and visibility of the item,
// but `Item_` has its type and contents but no span. 
// So we need to look at both to get a useful Spanned ParseNode.
impl ExplainParse for Item {
    fn children(&self) -> Vec<Spanned<ParseNode<String>>> {
        match self.node {
            Item_::ItemFn(ref decl, _, _, _, ref _generics, ref block) => {
                // FIXME: add in generics, etc.
                let mut children = decl.inputs.iter().map(|arg| arg.spanned_explain()).collect::<Vec<_>>();
                children.push(block.spanned_explain());
                children
            },
            _ => unreachable!()
        }
    }
    fn _explain(&self) -> String {
        match self.node {
            Item_::ItemFn(ref _decl, unsafety, constness, _, ref _generics, ref block) =>
                format!("A{}{}{}declaration of a function named {}",
                    if unsafety == Unsafety::Unsafe { "n unsafe " } else { " safe " },
                    if constness == Constness::Const { "const "} else { "" },
                    if self.vis == Visibility::Public { "public "} else { "inherited-visibility "},
                    self.ident.name.as_str()),
            _ => unreachable!()
        }
    }
}

impl ExplainParse for Arg {
    fn children(&self) -> Vec<Spanned<ParseNode<String>>> {
        vec!(self.ty.spanned_explain(), self.pat.spanned_explain())
    }
    fn _explain(&self) -> String {
        "An argument".to_string()
    }
}


fn combine_spans(s1: Span, s2: Span) -> Span {
    let lo = cmp::min(s1.lo.to_usize(), s2.lo.to_usize());
    let hi = cmp::max(s1.hi.to_usize(), s2.hi.to_usize());
    Span { lo: BytePos::from_usize(lo), hi: BytePos::from_usize(hi), expn_id: NO_EXPANSION}
}
