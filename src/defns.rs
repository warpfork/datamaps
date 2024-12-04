use indexmap::IndexMap;

struct Schema<'a> {
    types: IndexMap<&'a str, &'a Type>,
}

enum Type {}

trait TypeTrait {
    fn name() -> String {
        todo!();
    }
}
