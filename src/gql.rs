use bson::oid::ObjectId;
use juniper::FieldResult;

use super::Context;
use crate::models::Thing;
use crate::models::Prop;

pub struct Query;
pub struct Mutations;

graphql_object!(Prop: Context |&self| {
    field id() -> String { if let Some(ref id) = self.id { id.to_hex() } else { "".into() } }
    field thing_id() -> String { self.thing_id.to_hex() }
    field kind() -> &str { self.kind.as_str() }
    field value() -> &str { self.value.as_str() }
});

graphql_object!(Thing: Context |&self| {
    field id() -> String { if let Some(ref id) = self.id { id.to_hex() } else { "".into() } }
    field name() -> &str { self.name.as_str() }
    field props(&executor) -> Vec<Prop> {
        let context = executor.context();
        context.db.list_props().unwrap().iter().filter(|&p| p.thing_id == self.id.unwrap()).collect().iter().map(|&p| p).collect()
    }
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
    field saveProp(&executor,
        id: Option<String>,
        thing_id: String,
        kind: String,
        value: String,
    ) -> FieldResult<Option<Prop>> {
        let context = executor.context();
        let id = id.map(|id| ObjectId::with_string(&id)).map_or(Ok(None), |v| v.map(Some))?;
        let thing_id = ObjectId::with_string(&thing_id).unwrap();

        let prop = Prop {
            id: id,
            thing_id: thing_id,
            kind: kind,
            value: value,
        };
        Ok(context.db.save_prop(prop)?)
    }

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
