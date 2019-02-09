use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Thing {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(rename = "_id")]
    pub thing_id: ObjectId,

    #[serde(default)]
    pub kind: String,

    #[serde(default)]
    pub value: String,
}
