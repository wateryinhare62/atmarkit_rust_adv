use askama::Template;
use crate::store;

#[derive(Template)]
#[template(path = "item.html")]
struct ItemTemplate<'a> {
    item: &'a store::Task,
}

pub fn render_item(item: &store::Task) -> String {
    let template = ItemTemplate { item: &item };
    template.render().unwrap()
}
