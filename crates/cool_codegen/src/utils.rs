use cool_collections::SmallString;
use cool_lexer::symbols::sym;
use cool_resolve::ItemPath;

pub fn mangle_item_path<'a, P>(path: P) -> SmallString
where
    P: Into<ItemPath<'a>>,
{
    let path: ItemPath = path.into();

    if path.last() == sym::MAIN {
        return SmallString::from("main");
    }

    let path = path.as_symbol_slice();

    let Some((&first, others)) = path.split_first() else {
        return SmallString::new();
    };

    let mut result = SmallString::from(first.as_str());

    for other in others {
        result.push_str("__");
        result.push_str(other.as_str());
    }

    result
}
