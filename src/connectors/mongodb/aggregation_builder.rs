use std::collections::HashSet;
use serde_json::{Value as JsonValue, Map as JsonMap};
use bson::{Bson, bson, DateTime as BsonDateTime, doc, Document, oid::ObjectId, Regex as BsonRegex};
use chrono::{Date, NaiveDate, Utc, DateTime};
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::value::Value;
use crate::error::ActionError;


#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum QueryPipelineType {
    Unique,
    First,
    Many
}

pub trait ToBsonValue {
    fn to_bson_value(&self) -> Bson;
}

impl ToBsonValue for Value {
    fn to_bson_value(&self) -> Bson {
        match self {
            Value::Null => {
                Bson::Null
            }
            Value::ObjectId(val) => {
                Bson::ObjectId(ObjectId::parse_str(val.as_str()).unwrap())
            }
            Value::Bool(val) => {
                Bson::Boolean(*val)
            }
            Value::I8(val) => {
                Bson::Int32(*val as i32)
            }
            Value::I16(val) => {
                Bson::Int32(*val as i32)
            }
            Value::I32(val) => {
                Bson::Int32(*val)
            }
            Value::I64(val) => {
                Bson::Int64(*val)
            }
            Value::I128(val) => {
                Bson::Int64(*val as i64)
            }
            Value::U8(val) => {
                Bson::Int32(*val as i32)
            }
            Value::U16(val) => {
                Bson::Int32(*val as i32)
            }
            Value::U32(val) => {
                Bson::Int64(*val as i64)
            }
            Value::U64(val) => {
                Bson::Int64(*val as i64)
            }
            Value::U128(val) => {
                Bson::Int64(*val as i64)
            }
            Value::F32(val) => {
                Bson::from(val)
            }
            Value::F64(val) => {
                Bson::from(val)
            }
            Value::String(val) => {
                Bson::String(val.clone())
            }
            Value::Decimal(val) => {
                todo!()
            }
            Value::Date(val) => {
                Bson::DateTime(BsonDateTime::parse_rfc3339_str(val.format("%Y-%m-%d").to_string()).unwrap())
            }
            Value::DateTime(val) => {
                Bson::DateTime(BsonDateTime::from(*val))
            }
            Value::Vec(val) => {
                Bson::Array(val.iter().map(|i| { i.to_bson_value() }).collect())
            }
            Value::Map(val) => {
                let mut doc = doc!{};
                for (k, v) in val {
                    doc.insert(k.to_string(), v.to_bson_value());
                }
                Bson::Document(doc)
            }
            Value::Object(obj) => {
                panic!()
            }
        }
    }
}

fn parse_object_id(value: &JsonValue) -> Result<Bson, ActionError> {
    match value.as_str() {
        Some(val) => {
            match ObjectId::parse_str(val) {
                Ok(oid) => {
                    Ok(Bson::ObjectId(oid))
                }
                Err(_) => {
                    Err(ActionError::wrong_input_type())
                }
            }
        }
        None => {
            Err(ActionError::wrong_input_type())
        }
    }
}


fn has_i_mode(map: &JsonMap<String, JsonValue>) -> bool {
    match map.get("mode") {
        Some(val) => {
            if val.is_string() {
                return val.as_str().unwrap() == "caseInsensitive"
            } else {
                false
            }
        }
        None => {
            false
        }
    }
}

fn parse_string(value: &JsonValue) -> Result<Bson, ActionError> {
    match value.as_str() {
        Some(val) => {
            Ok(Bson::String(val.to_string()))
        }
        None => {
            Err(ActionError::wrong_input_type())
        }
    }
}

fn parse_bool(value: &JsonValue) -> Result<Bson, ActionError> {
    match value.as_bool() {
        Some(val) => {
            Ok(Bson::Boolean(val))
        }
        None => {
            Err(ActionError::wrong_input_type())
        }
    }
}

fn parse_i64(value: &JsonValue) -> Result<Bson, ActionError> {
    if value.is_i64() {
        Ok(Bson::Int64(value.as_i64().unwrap()))
    } else if value.is_u64() {
        Ok(Bson::Int64(value.as_u64().unwrap() as i64))
    } else if value.is_f64() {
        Ok(Bson::Int64(value.as_f64().unwrap() as i64))
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn parse_f64(value: &JsonValue) -> Result<Bson, ActionError> {
    if value.is_i64() {
        Ok(Bson::Double(value.as_i64().unwrap() as f64))
    } else if value.is_u64() {
        Ok(Bson::Double(value.as_u64().unwrap() as f64))
    } else if value.is_f64() {
        Ok(Bson::Double(value.as_f64().unwrap()))
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn parse_date(value: &JsonValue) -> Result<Bson, ActionError> {
    if value.is_string() {
        match NaiveDate::parse_from_str(&value.as_str().unwrap(), "%Y-%m-%d") {
            Ok(naive_date) => {
                let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                let val = Value::Date(date);
                Ok(val.to_bson_value())
            }
            Err(_) => {
                Err(ActionError::wrong_date_format())
            }
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn parse_datetime(value: &JsonValue) -> Result<Bson, ActionError> {
    if value.is_string() {
        match DateTime::parse_from_rfc3339(&value.as_str().unwrap()) {
            Ok(fixed_offset_datetime) => {
                let datetime: DateTime<Utc> = fixed_offset_datetime.with_timezone(&Utc);
                let value = Value::DateTime(datetime);
                Ok(value.to_bson_value())
            }
            Err(_) => {
                Err(ActionError::wrong_datetime_format())
            }
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn parse_enum(value: &JsonValue, enum_name: &str, graph: &Graph) -> Result<Bson, ActionError> {
    if value.is_string() {
        let str = value.as_str().unwrap();
        let r#enum = graph.r#enum(enum_name);
        if r#enum.contains(&str.to_string()) {
            Ok(Bson::String(str.to_string()))
        } else {
            Err(ActionError::undefined_enum_value())
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn parse_bson_where_entry(field_type: &FieldType, value: &JsonValue, graph: &Graph) -> Result<Bson, ActionError> {
    return match field_type {
        FieldType::Undefined => {
            panic!()
        }
        FieldType::ObjectId => {
            if value.is_string() {
                parse_object_id(value)
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let oid = parse_object_id(value)?;
                            result.insert("$eq", oid);
                        }
                        "not" => {
                            let oid = parse_object_id(value)?;
                            result.insert("$eq", oid);
                        }
                        "gt" => {
                            let oid = parse_object_id(value)?;
                            result.insert("$gt", oid);
                        }
                        "gte" => {
                            let oid = parse_object_id(value)?;
                            result.insert("$gt", oid);
                        }
                        "lt" => {
                            let oid = parse_object_id(value)?;
                            result.insert("$gt", oid);
                        }
                        "lte" => {
                            let oid = parse_object_id(value)?;
                            result.insert("$gt", oid);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_object_id(val)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_object_id(val)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::Bool => {
            if value.is_boolean() {
                Ok(Bson::Boolean(value.as_bool().unwrap()))
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_bool(value)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_bool(value)?;
                            result.insert("$eq", b);
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 => {
            if value.is_i64() {
                Ok(Bson::Int64(value.as_i64().unwrap()))
            } else if value.is_u64() {
                Ok(Bson::Int64(value.as_u64().unwrap() as i64))
            } else if value.is_f64() {
                Ok(Bson::Int64(value.as_f64().unwrap() as i64))
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_i64(value)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_i64(value)?;
                            result.insert("$eq", b);
                        }
                        "gt" => {
                            let oid = parse_i64(value)?;
                            result.insert("$gt", oid);
                        }
                        "gte" => {
                            let oid = parse_i64(value)?;
                            result.insert("$gt", oid);
                        }
                        "lt" => {
                            let oid = parse_i64(value)?;
                            result.insert("$gt", oid);
                        }
                        "lte" => {
                            let oid = parse_i64(value)?;
                            result.insert("$gt", oid);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_i64(val)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_i64(val)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::F32 | FieldType::F64 => {
            if value.is_i64() {
                Ok(Bson::Double(value.as_i64().unwrap() as f64))
            } else if value.is_u64() {
                Ok(Bson::Double(value.as_u64().unwrap() as f64))
            } else if value.is_f64() {
                Ok(Bson::Double(value.as_f64().unwrap()))
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_f64(value)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_f64(value)?;
                            result.insert("$eq", b);
                        }
                        "gt" => {
                            let oid = parse_f64(value)?;
                            result.insert("$gt", oid);
                        }
                        "gte" => {
                            let oid = parse_f64(value)?;
                            result.insert("$gt", oid);
                        }
                        "lt" => {
                            let oid = parse_f64(value)?;
                            result.insert("$gt", oid);
                        }
                        "lte" => {
                            let oid = parse_f64(value)?;
                            result.insert("$gt", oid);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_f64(val)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_f64(val)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::Decimal => {
            todo!()
        }
        FieldType::String => {
            if value.is_string() {
                Ok(Bson::String(value.as_str().unwrap().to_string()))
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_string(value)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_string(value)?;
                            result.insert("$eq", b);
                        }
                        "gt" => {
                            let oid = parse_string(value)?;
                            result.insert("$gt", oid);
                        }
                        "gte" => {
                            let oid = parse_string(value)?;
                            result.insert("$gt", oid);
                        }
                        "lt" => {
                            let oid = parse_string(value)?;
                            result.insert("$gt", oid);
                        }
                        "lte" => {
                            let oid = parse_string(value)?;
                            result.insert("$gt", oid);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_string(val)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_string(val)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "contains" => {
                            let bson_regex = BsonRegex {
                                pattern: regex::escape(parse_string(value)?.as_str().unwrap()),
                                options: if has_i_mode(map) { "i".to_string() } else { "".to_string() }
                            };
                            let regex = Bson::RegularExpression(bson_regex);
                            result.insert("$regex", regex);
                        }
                        "startsWith" => {
                            let bson_regex = BsonRegex {
                                pattern: "^".to_string() + &*regex::escape(parse_string(value)?.as_str().unwrap()),
                                options: if has_i_mode(map) { "i".to_string() } else { "".to_string() }
                            };
                            let regex = Bson::RegularExpression(bson_regex);
                            result.insert("$regex", regex);
                        }
                        "endsWith" => {
                            let bson_regex = BsonRegex {
                                pattern: regex::escape(parse_string(value)?.as_str().unwrap()) + "$",
                                options: if has_i_mode(map) { "i".to_string() } else { "".to_string() }
                            };
                            let regex = Bson::RegularExpression(bson_regex);
                            result.insert("$regex", regex);
                        }
                        "matches" => {
                            let bson_regex = BsonRegex {
                                pattern: parse_string(value)?.as_str().unwrap().to_string(),
                                options: if has_i_mode(map) { "i".to_string() } else { "".to_string() }
                            };
                            let regex = Bson::RegularExpression(bson_regex);
                            result.insert("$regex", regex);
                        }
                        "mode" => { }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::Date => {
            if value.is_string() {
                parse_date(value)
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_date(value)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_date(value)?;
                            result.insert("$eq", b);
                        }
                        "gt" => {
                            let oid = parse_date(value)?;
                            result.insert("$gt", oid);
                        }
                        "gte" => {
                            let oid = parse_date(value)?;
                            result.insert("$gt", oid);
                        }
                        "lt" => {
                            let oid = parse_date(value)?;
                            result.insert("$gt", oid);
                        }
                        "lte" => {
                            let oid = parse_date(value)?;
                            result.insert("$gt", oid);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_date(val)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_date(val)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::DateTime => {
            if value.is_string() {
                parse_datetime(value)
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_datetime(value)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_datetime(value)?;
                            result.insert("$eq", b);
                        }
                        "gt" => {
                            let oid = parse_datetime(value)?;
                            result.insert("$gt", oid);
                        }
                        "gte" => {
                            let oid = parse_datetime(value)?;
                            result.insert("$gt", oid);
                        }
                        "lt" => {
                            let oid = parse_datetime(value)?;
                            result.insert("$gt", oid);
                        }
                        "lte" => {
                            let oid = parse_datetime(value)?;
                            result.insert("$gt", oid);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_datetime(val)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_datetime(val)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::Enum(enum_name) => {
            if value.is_string() {
                parse_enum(value, enum_name, graph)
            } else if value.is_object() {
                let map = value.as_object().unwrap();
                let mut result = doc!{};
                for (key, value) in map {
                    match key.as_str() {
                        "equals" => {
                            let b = parse_enum(value, enum_name, graph)?;
                            result.insert("$eq", b);
                        }
                        "not" => {
                            let b = parse_enum(value, enum_name, graph)?;
                            result.insert("$eq", b);
                        }
                        "in" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_enum(value, enum_name, graph)?);
                                    }
                                    result.insert("$in", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        "notIn" => {
                            match value.as_array() {
                                Some(arr_val) => {
                                    let mut arr: Vec<Bson> = Vec::new();
                                    for val in arr_val {
                                        arr.push(parse_enum(value, enum_name, graph)?);
                                    }
                                    result.insert("$nin", arr);
                                }
                                None => {
                                    return Err(ActionError::wrong_input_type());
                                }
                            }
                        }
                        &_ => {
                            return Err(ActionError::wrong_input_type());
                        }
                    }
                }
                Ok(Bson::Document(result))
            } else {
                Err(ActionError::wrong_input_type())
            }
        }
        FieldType::Vec(_) => {
            panic!()
        }
        FieldType::Map(_) => {
            panic!()
        }
        FieldType::Object(_) => {
            panic!()
        }
    }
}

pub(crate) fn build_where_input(model: &Model, graph: &Graph, r#where: Option<&JsonValue>) -> Result<Document, ActionError> {
    if let None = r#where { return Ok(doc!{}); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() { return Err(ActionError::wrong_json_format()); }
    let r#where = r#where.as_object().unwrap();
    let mut doc = doc!{};
    for (key, value) in r#where.iter() {
        if !model.query_keys().contains(key) {
            return Err(ActionError::keys_unallowed());
        }
        let field = model.field(key).unwrap();
        let db_key = field.column_name();
        let bson_result = parse_bson_where_entry(&field.field_type, value, graph);
        match bson_result {
            Ok(bson) => {
                doc.insert(db_key, bson);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(doc)
}

fn build_lookup_inputs(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    include: &JsonValue,
) -> Result<Vec<Document>, ActionError> {
    let include = include.as_object();
    if include.is_none() {
        let model_name = &model.name;
        return Err(ActionError::invalid_query_input(format!("'include' on model '{model_name}' is not an object. Please check your input.")));
    }
    let include = include.unwrap();
    let mut retval: Vec<Document> = vec![];
    for (key, value) in include.iter() {
        let relation = model.relation(key);
        if relation.is_none() {
            let model_name = &model.name;
            return Err(ActionError::invalid_query_input(format!("Relation '{key}' on model '{model_name}' is not exist. Please check your input.")));
        }
        let relation = relation.unwrap();
        let relation_name = &relation.name;
        let relation_model_name = &relation.model;
        let relation_model = graph.model(relation_model_name);
        if value.is_boolean() || value.is_object() {
            if relation.through.is_none() { // without join table
                let mut let_value = doc!{};
                let mut eq_values: Vec<Document> = vec![];
                for (index, field_name) in relation.fields.iter().enumerate() {
                    let field_name = model.field(field_name).unwrap().column_name();
                    let reference_name = relation.references.get(index).unwrap();
                    let relation_model = graph.model(&relation.model);
                    let reference_name_column_name = relation_model.field(reference_name).unwrap().column_name();
                    let_value.insert(reference_name, format!("${field_name}"));
                    eq_values.push(doc!{"$eq": [format!("${reference_name_column_name}"), format!("$${reference_name}")]});
                }
                let mut inner_pipeline = if value.is_object() {
                    build_query_pipeline_from_json(relation_model, graph, r#type, mutation_mode, value)?
                } else {
                    vec![]
                };
                let inner_match = inner_pipeline.iter().find(|v| v.get("$match").is_some());
                let has_inner_match = inner_match.is_some();
                let mut inner_match = if has_inner_match {
                    inner_match.unwrap().clone()
                } else {
                    doc!{"$match": {}}
                };
                let mut inner_match_inner = inner_match.get_mut("$match").unwrap().as_document_mut().unwrap();
                if inner_match_inner.get("$expr").is_none() {
                    inner_match_inner.insert("$expr", doc!{});
                }
                if inner_match_inner.get("$expr").unwrap().as_document().unwrap().get("$and").is_none() {
                    inner_match_inner.get_mut("$expr").unwrap().as_document_mut().unwrap().insert("$and", vec![] as Vec<Document>);
                }
                inner_match_inner.get_mut("$expr").unwrap().as_document_mut().unwrap().get_mut("$and").unwrap().as_array_mut().unwrap().extend(eq_values.iter().map(|item| Bson::Document(item.clone())));
                if has_inner_match {
                    let index = inner_pipeline.iter().position(|v| v.get("$match").is_some()).unwrap();
                    inner_pipeline.remove(index);
                    inner_pipeline.insert(index, inner_match);
                } else {
                    inner_pipeline.insert(0, inner_match);
                }
                let lookup = doc!{"$lookup": {
                    "from": &relation_model.table_name,
                    "as": key,
                    "let": let_value,
                    "pipeline": inner_pipeline
                }};
                retval.push(lookup);
            } else { // with join table
                let join_model = graph.model(relation.through.as_ref().unwrap());
                let local_relation_on_join_table = join_model.relation(relation.fields.get(0).unwrap()).unwrap();
                let foreign_relation_on_join_table = join_model.relation(relation.references.get(0).unwrap()).unwrap();
                let foreign_model_name = &foreign_relation_on_join_table.model;
                let foreign_model = graph.model(foreign_model_name);
                let mut outer_let_value = doc!{};
                let mut outer_eq_values: Vec<Document> = vec![];
                let mut inner_let_value = doc!{};
                let mut inner_eq_values: Vec<Document> = vec![];
                for (index, join_table_field_name) in local_relation_on_join_table.fields.iter().enumerate() {
                    let local_unique_field_name = local_relation_on_join_table.references.get(index).unwrap();
                    let local_unique_field_column_name = model.field(local_unique_field_name).unwrap().column_name();
                    outer_let_value.insert(join_table_field_name, format!("${local_unique_field_column_name}"));
                    outer_eq_values.push(doc!{"$eq": [format!("${join_table_field_name}"), format!("$${join_table_field_name}")]});
                }
                for (index, join_table_reference_name) in foreign_relation_on_join_table.fields.iter().enumerate() {
                    let foreign_unique_field_name = foreign_relation_on_join_table.references.get(index).unwrap();
                    let foreign_unique_field_column_name = foreign_model.field(foreign_unique_field_name).unwrap().column_name();
                    inner_let_value.insert(join_table_reference_name, format!("${join_table_reference_name}"));
                    inner_eq_values.push(doc!{"$eq": [format!("${foreign_unique_field_column_name}"), format!("$${join_table_reference_name}")]});
                }
                let target = doc!{
                    "$lookup": {
                        "from": join_model.table_name(),
                        "as": relation_name,
                        "let": outer_let_value,
                        "pipeline": [{
                            "$match": {
                                "$expr": {
                                    "$and": outer_eq_values
                                }
                            }
                        }, {
                            "$lookup": {
                                "from": foreign_model.table_name(),
                                "as": relation_name,
                                "let": inner_let_value,
                                "pipeline": [{
                                    "$match": {
                                        "$expr": {
                                            "$and": inner_eq_values
                                        }
                                    }
                                }]
                            }
                        }, {
                            "$unwind": {
                                "path": format!("${relation_name}")
                            }
                        }, {
                            "$replaceRoot": {
                                "newRoot": format!("${relation_name}")
                            }
                        }]
                    }
                };
                println!("generated lookup for join table: {:?}", target);
                retval.push(target);
            }
        } else {
            let model_name = &model.name;
            return Err(ActionError::invalid_query_input(format!("Relation '{key}' on model '{model_name}' has a unrecognized value. It's either a boolean or an object. Please check your input.")));
        }
    }
    Ok(retval)
}

fn build_query_pipeline(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    r#where: Option<&JsonValue>,
    order_by: Option<&JsonValue>,
    take: Option<usize>,
    skip: Option<usize>,
    page_size: Option<usize>,
    page_number: Option<usize>,
    include: Option<&JsonValue>,
    select: Option<&JsonValue>,
) -> Result<Vec<Document>, ActionError> {
    let mut retval: Vec<Document> = vec![];
    // $match
    let r#match = build_where_input(model, graph, r#where)?;
    if !r#match.is_empty() {
        retval.push(doc!{"$match": r#match});
    }
    // $sort

    // $skip and $limit
    if page_size.is_some() && page_number.is_some() {
        retval.push(doc!{"$skip": ((page_number.unwrap() - 1) * page_size.unwrap()) as i64});
        retval.push(doc!{"limit": page_size.unwrap() as i64});
    } else {
        if skip.is_some() {
            retval.push(doc!{"$skip": skip.unwrap() as i64});
        }
        if take.is_some() {
            retval.push(doc!{"$limit": skip.unwrap() as i64});
        }
    }
    // $project
    // $lookup
    if include.is_some() {
        let mut lookups = build_lookup_inputs(model, graph, r#type, mutation_mode, include.unwrap())?;
        if !lookups.is_empty() {
            retval.append(&mut lookups);
        }
    }
    Ok(retval)
}

fn unwrap_usize(value: Option<&JsonValue>) -> Option<usize> {
    match value {
        Some(value) => Some(value.as_u64().unwrap() as usize),
        None => None
    }
}

pub(crate) fn validate_where_unique(model: &Model, r#where: &Option<&JsonValue>) -> Result<(), ActionError> {
    if r#where.is_none() {
        return Err(ActionError::missing_input_section());
    }
    let r#where = r#where.unwrap();
    if !r#where.is_object() {
        return Err(ActionError::wrong_json_format());
    }
    let values = r#where.as_object().unwrap();
    // see if key is valid
    let set_vec: Vec<String> = values.keys().map(|k| k.clone()).collect();
    let set = HashSet::from_iter(set_vec.iter().map(|k| k.clone()));
    if !model.unique_query_keys().contains(&set) {
        return Err(ActionError::field_is_not_unique())
    }
    Ok(())
}

/// Build MongoDB aggregation pipeline for querying.
/// # Arguments
///
/// * `mutation_mode` - When mutation mode is true, `select` and `include` is ignored.
///
pub(crate) fn build_query_pipeline_from_json(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    json_value: &JsonValue
) -> Result<Vec<Document>, ActionError> {
    let json_value = json_value.as_object();
    if json_value.is_none() {
        return Err(ActionError::invalid_query_input("Query input should be an object."));
    }
    let json_value = json_value.unwrap();
    let r#where = json_value.get("where");
    if r#type == QueryPipelineType::Unique {
        validate_where_unique(model, &r#where)?;
    }
    let order_by = json_value.get("orderBy");
    let take = unwrap_usize(json_value.get("take"));
    let skip = unwrap_usize(json_value.get("skip"));
    let page_number = unwrap_usize(json_value.get("pageNumber"));
    let page_size = unwrap_usize(json_value.get("pageSize"));
    let include = if !mutation_mode { json_value.get("include") } else { None };
    let select = if !mutation_mode { json_value.get("select") } else { None };
    build_query_pipeline(model, graph, r#type, mutation_mode, r#where, order_by, take, skip, page_size, page_number, include, select)
}