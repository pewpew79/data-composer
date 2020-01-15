use std::fs;

use serde_json::Value;
use mongodb::{Client, Collection};
use bson::ordered::OrderedDocument;



fn read_json(file_path: &str) -> serde_json::Value {
    fs::read_to_string(file_path).and_then(
        |content| Ok(serde_json::from_str(&content).expect("Error convert JSON string into Value!"))
    ).unwrap_or(Value::Null)
}


fn convert_to_doc(d: &serde_json::Value) -> OrderedDocument {
    let result: bson::Bson = d.clone().into();  // Maybe need to optimize ...
    result.as_document().expect("Error converting JSON Value into Bson filter!").clone()
}


fn mongo_get_coll(mongo_uri: &str, db_name: &str, coll_name: &str) -> Collection {
    let client =  Client::with_uri_str(&mongo_uri).ok().expect("Failed to initialize client!");
    let db = client.database(db_name);
    db.collection(coll_name)
}


fn mongo_save(coll: &Collection, data: serde_json::Value, data_root_key: &str) {
    let docs: Vec<OrderedDocument> = data[data_root_key].as_array().unwrap().iter()
        .map(convert_to_doc).collect();

    coll.insert_many(docs, None).expect("Error write doc into Mongo!");
}


fn mongo_get(coll: &Collection, filter: OrderedDocument) -> Vec<OrderedDocument> {
    match coll.find(filter, None) {
        Ok(cursor) => cursor.map(|doc| doc.unwrap()).collect::<Vec<_>>(),
        Err(_err) => Vec::new()
    }
}




pub fn main_test() {
    let mongo_uri = "mongodb://localhost:27017";
    let mongo_db_name = "test_db";
    let mongo_db_coll = "test_coll";

    let mongo_coll = mongo_get_coll(mongo_uri, mongo_db_name, mongo_db_coll);
    let data = read_json("test.json");

    mongo_save(&mongo_coll, data, "data");

//    let filter_value: serde_json::Value = serde_json::from_str(r#"{"phones": 10}"#).unwrap();
    let filter_value: serde_json::Value = serde_json::from_str(r#"{"phones": {"$gte": 60}}"#).unwrap();
    let filter: bson::Bson = filter_value.into();
    let filter = filter.as_document().expect("Error converting JSON Value into Bson filter!");

    let results = mongo_get(&mongo_coll, filter.clone());
    println!("{:?}", results);
}
