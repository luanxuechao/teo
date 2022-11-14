use std::collections::HashMap;
use std::sync::Arc;
use maplit::hashmap;
use crate::core::field::Field;
use crate::core::model::Model;
use crate::core::property::Property;
use crate::core::relation::Relation;
use crate::core::tson::Value;
use crate::parser::ast::argument::Argument;
use crate::parser::std::constants::EnvObject;

pub(crate) type FieldDecorator = fn(args: Vec<Argument>, field: &mut Field);

pub(crate) type RelationDecorator = fn(args: Vec<Argument>, relation: &mut Relation);

pub(crate) type PropertyDecorator = fn(args: Vec<Argument>, property: &mut Property);

pub(crate) type ModelDecorator = fn(args: Vec<Argument>, model: &mut Model);

pub(crate) struct Container {
    pub(crate) objects: HashMap<String, Object>
}

impl Container {
    pub(crate) fn std_global_constants() -> Self {
        Self {
            objects: hashmap!{
                "ENV".to_owned() => Object::Env(EnvObject {})
            }
        }
    }
}

pub(crate) enum Object {
    FieldDecorator(FieldDecorator),
    RelationDecorator(RelationDecorator),
    PropertyDecorator(PropertyDecorator),
    ModelDecorator(ModelDecorator),
    Container(Container),
    Env(EnvObject),
    Value(Value),
}