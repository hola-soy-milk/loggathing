use bson::oid::ObjectId;
use juniper::FieldResult;

use super::Context;
use crate::models::Thing;

pub struct Query;
pub struct Mutations;

graphql_object!(Thing: Context |&self| {
    field id() -> String { if let Some(ref id) = self.id { id.to_hex() } else { "".into() } }
    field name() -> &str { self.name.as_str() }
});

graphql_object!(Query: Context |&self| {
  field apiVersion() -> &str {
    "1.0"
  }

    field things(&executor) -> FieldResult<Vec<Thing>> {
    let context = executor.context();
        Ok(context.db.list_things()?)
    }

  field thing(&executor, id: String) -> FieldResult<Option<Thing>> {
    let context = executor.context();
    Ok(context.db.get_thing(&id)?)
  }
});

graphql_object!(Mutations: Context |&self| {
    field saveThing(&executor,
        id: Option<String>,
        name: String,
    ) -> FieldResult<Option<Thing>> {
        let context = executor.context();
        let id = id.map(|id| ObjectId::with_string(&id)).map_or(Ok(None), |v| v.map(Some))?;

        let thing = Thing {
            id: id,
            name: name,
        };

        Ok(context.db.save_thing(thing)?)
    }
});
