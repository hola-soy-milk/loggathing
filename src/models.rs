use bson::oid::ObjectId;
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug)]
pub struct Thing {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Prop {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub thing_id: String,

    #[serde(default)]
    pub kind: String,

    #[serde(default)]
    pub value: String,
}
