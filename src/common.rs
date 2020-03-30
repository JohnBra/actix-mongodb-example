use serde::{Serialize, Deserialize, Serializer};
use bson::Document;
use bson::ordered::OrderedDocument;
use mongodb::Cursor;
use bson::oid::ObjectId;
use std::env;


pub fn get_env_or_panic(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_e) => panic!("Couldn't interpret required '{}' env variable. Terminating...", key)
    }
}

pub trait CursorAsVec {
    fn as_vec<'a, T: Serialize + Deserialize<'a>>(&mut self) -> Vec<T>;
}

impl CursorAsVec for Cursor {
    fn as_vec<'a, T: Serialize + Deserialize<'a>>(&mut self) -> Vec<T> {
        self.map(|item| {
            let doc: Document = item.unwrap();
            let bson = bson::Bson::Document(doc);
            return bson::from_bson(bson).unwrap();
        }).collect()
    }
}


pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none()
    }
}

pub fn struct_to_document<'a, T: Sized + Serialize + Deserialize<'a>>(
    t: &T
) -> Option<OrderedDocument> {
    let mid: Option<OrderedDocument> = bson::to_bson(t)
        .ok()
        .map(|x| x.as_document().unwrap().to_owned());

    mid.map(|mut doc| {
        let keys = doc.keys();
        let rm: Vec<String> = keys
            .filter(|k| doc.is_null(k))
            .map(|x| x.to_owned())
            .collect();
        // remove null value fields
        for x in rm {
            doc.remove(&x);
        }
        doc
    })
}
