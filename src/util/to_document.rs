use bson::{to_bson, Document};
use serde::Serialize;

pub fn to_doc<T>(value: &T) -> Document
where
    T: Serialize,
{
    to_bson(value).unwrap().as_document().unwrap().clone()
}
