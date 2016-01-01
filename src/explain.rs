use syntax::ast::{Stmt_, Decl_, Pat_, Ty_, Expr_, Block, Mod, Item, Item_};
use syntax::codemap::Spanned;

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
        Spanned { node: self.node.explain(), span: self.span }
    }
}

impl <T> SpannedExplainParse for Spanned<T> where T: ExplainParse {
    fn spanned_explain(&self) -> Spanned<ParseNode<String>> {
        Spanned { node: self.node.explain(), span: self.span }
    }
}


pub trait ExplainParse {
    fn _explain(&self) -> String;
    fn explain(&self) -> ParseNode<String> {
        ParseNode { value: self._explain(), children: vec!() }
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
    fn explain(&self) -> ParseNode<String> {
        ParseNode { 
            value: self._explain(),
            children: vec!(self.stmts[0].spanned_explain())
        }
    }
    fn _explain(&self) -> String {
        format!("A block")
    }
}

impl ExplainParse for Mod {
    fn explain(&self) -> ParseNode<String> {
        ParseNode { 
            value: self._explain(),
            children: vec!(self.items[0].spanned_explain())
        }
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

impl ExplainParse for Item_ {
    fn explain(&self) -> ParseNode<String> {
        let children = match self {
            &Item_::ItemFn(ref _decl, _, _, _, ref _generics, ref block) =>
                // FIXME: add in declaration, generics, etc.
                vec!(block.spanned_explain()),
            _ => vec!()
        };
        ParseNode { 
            value: self._explain(),
            children: children
        }
    }
    fn _explain(&self) -> String {
        match self {
            &Item_::ItemFn(ref _decl, _, _, _, ref _generics, ref block) =>
                format!("{}", "A function declaration"),
            _ => unreachable!()
        }
    }
}