use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::query::query_raw::QueryRawItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn query_raw(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( QueryRawItem::new(value.clone()))
}
