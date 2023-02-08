use crate::ast::ItemAst;
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct ModuleAst {
    pub span: Span,
    pub name: Symbol,
    pub items: Vec<ItemAst>,
}

/*
utils :: module {
    export heyooo :: fn() {
        println("Heyooo");
    };
};

main :: fn() {
    import utils.heyooo;

    import {
        utils.heyooo as bababoooo,
        utils.{self, heyooo as huuuuu};
    };

    i32 ::  utils.String;

    utils.heyooo();
}

*/
