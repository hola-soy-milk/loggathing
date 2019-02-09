use bson::{from_bson, oid::ObjectId, to_bson, Bson, Document};
use mongodb::{
    coll::options::{FindOneAndUpdateOptions, ReturnDocument, UpdateOptions},
    coll::Collection,
    cursor::Cursor,
    db::ThreadedDatabase,
    Client, ThreadedClient,
};

use crate::error::Error;
use crate::models::Thing;
use crate::models::Prop;

pub struct Db {
    client: Client,
    db_name: String,
}

impl Db {
    pub fn new<S>(db_name: S) -> Db
    where
        S: ToString,
    {
        let db_name = db_name.to_string();
        let client = Client::connect("localhost", 27017).expect("Failed to initialize client.");
        Db { client, db_name }
    }

    pub fn list_things(&self) -> Result<Vec<Thing>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("things");
        let cursor = coll.find(None, None)?;
        let res: Result<Vec<_>, _> = cursor
            .map(|row| row.and_then(|item| Ok(from_bson::<Thing>(Bson::Document(item))?)))
            .collect();

        Ok(res?)
    }

    pub fn list_props(&self) -> Result<Vec<Prop>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("props");
        let cursor = coll.find(None, None)?;
        let res: Result<Vec<_>, _> = cursor
            .map(|row| row.and_then(|item| Ok(from_bson::<Prop>(Bson::Document(item))?)))
            .collect();

        Ok(res?)
    }

    pub fn get_thing(&self, id: &str) -> Result<Option<Thing>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("things");
        let cursor: Option<Document> = coll.find_one(Some(doc! { "_id": ObjectId::with_string(id)? }), None)?;
        cursor
            .map(|doc| Ok(from_bson::<Thing>(Bson::Document(doc))?))
            .map_or(Ok(None), |v| v.map(Some))
    }

    pub fn get_prop(&self, id: &str) -> Result<Option<Prop>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("props");
        let cursor: Option<Document> = coll.find_one(Some(doc! { "_id": ObjectId::with_string(id)? }), None)?;
        cursor
            .map(|doc| Ok(from_bson::<Prop>(Bson::Document(doc))?))
            .map_or(Ok(None), |v| v.map(Some))
    }

    pub fn save_thing(&self, thing: Thing) -> Result<Option<Thing>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("things");

        if let Bson::Document(mut doc) = to_bson(&thing)? {
            doc.remove("_id");
            if let Some(ref id) = thing.id {
                let filter = doc!{ "_id": Bson::ObjectId(id.clone()) };
                let write_options = FindOneAndUpdateOptions {
                    return_document: Some(ReturnDocument::After),
                    ..Default::default()
                };
                let res = coll.find_one_and_replace(filter, doc, Some(write_options))?;
                if let Some(res) = res {
                    Ok(Some(from_bson::<Thing>(Bson::Document(res))?))
                } else {
                    Err(Error::Custom("No data returned after update".into()))
                }
            } else {
                let res = coll.insert_one(doc, None)?;

                if let Some(exception) = res.write_exception {
                    return Err(Error::from(exception));
                }

                if let Some(inserted_id) = res.inserted_id {
                    if let Bson::ObjectId(id) = inserted_id {
                        self.get_thing(&id.to_hex())
                    } else {
                        Err(Error::Custom("No valid id returned after insert".into()))
                    }
                } else {
                    Err(Error::Custom("No data returned after insert".into()))
                }
            }
        } else {
            Err(Error::Custom("Invalid document".into()))
        }
    }

    pub fn save_prop(&self, prop: Prop) -> Result<Option<Prop>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("props");

        if let Bson::Document(mut doc) = to_bson(&prop)? {
            doc.remove("_id");
            if let Some(ref id) = prop.id {
                let filter = doc!{ "_id": Bson::ObjectId(id.clone()) };
                let write_options = FindOneAndUpdateOptions {
                    return_document: Some(ReturnDocument::After),
                    ..Default::default()
                };
                let res = coll.find_one_and_replace(filter, doc, Some(write_options))?;
                if let Some(res) = res {
                    Ok(Some(from_bson::<Prop>(Bson::Document(res))?))
                } else {
                    Err(Error::Custom("No data returned after update".into()))
                }
            } else {
                let res = coll.insert_one(doc, None)?;

                if let Some(exception) = res.write_exception {
                    return Err(Error::from(exception));
                }

                if let Some(inserted_id) = res.inserted_id {
                    if let Bson::ObjectId(id) = inserted_id {
                        self.get_prop(&id.to_hex())
                    } else {
                        Err(Error::Custom("No valid id returned after insert".into()))
                    }
                } else {
                    Err(Error::Custom("No data returned after insert".into()))
                }
            }
        } else {
            Err(Error::Custom("Invalid document".into()))
        }
    }
}
