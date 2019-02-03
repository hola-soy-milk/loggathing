#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

#[macro_use] extern crate juniper;

use juniper::{FieldResult};

extern crate iron;
extern crate juniper_iron;
extern crate mount;

use iron::prelude::*;
use iron::status;

use mount::Mount;
use juniper::EmptyMutation;
use juniper_iron::GraphQLHandler;

use std::sync::Arc;

#[derive(GraphQLObject)]
#[graphql(description="Property of a thing")]
struct Prop {
    id: String,
    kind: String,
    value: String
}

#[derive(GraphQLInputObject)]
#[graphql(description="Property of a thing")]
struct NewProp {
    kind: String,
    value: String
}

#[derive(GraphQLObject)]
#[graphql(description="A thing to be logged")]
struct Thing {
    id: String,
    data: Vec<Prop>
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description="A thing to be logged")]
struct NewThing {
    name: String,
    data: Vec<Prop>
}

#[derive(Clone)]
pub struct Context {
    pub db: Arc<Db>,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "0.0.1"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // The executor is a special (optional) argument that allows accessing the context.
    field thing(&executor, id: String) -> FieldResult<Thing> {
        // Get the context from the executor.
        let context = executor.context();
        // Get a db connection.
        let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        let thing = connection.find_thing(&id)?;
        // Return the result.
        Ok(thing)
    }
});

struct Mutation;

graphql_object!(Mutation: Context |&self| {

    field createThing(&executor, new_thing: NewThing) -> FieldResult<Thing> {
        let db = executor.context().pool.get_connection()?;
        let thing: Thing = db.insert_thing(&new_thing)?;
        Ok(thing)
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;

fn context_factory(_: &mut Request) -> IronResult<()> {
    Ok(())
}

struct Root;

graphql_object!(Root: () |&self| {
    field foo() -> String {
        "Bar".to_owned()
    }
});

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        Root,
        EmptyMutation::<()>::new(),
    );

    mount.mount("/graphql", graphql_endpoint);

    let chain = Chain::new(mount);

    Iron::new(chain).http("0.0.0.0:8080").unwrap();
}
