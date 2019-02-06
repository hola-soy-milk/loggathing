use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Thing {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub kind: String,

    #[serde(default)]
    pub value: String,
}
