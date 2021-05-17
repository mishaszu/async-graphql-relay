use crate::tenant::Tenant;
use crate::user::User;
use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{
    EmptyMutation, EmptySubscription, InputValueResult, Interface, Object, Response, Scalar,
    ScalarType, Value,
};
use async_graphql_actix_web::Request;
use async_graphql_relay::{RelayGlobalID, RelayNodeEnum};
use std::fmt;

mod tenant;
mod user;

pub struct QueryRoot;

#[derive(RelayGlobalID)]
pub struct ID(pub String, pub SchemaNodeTypes);

#[derive(Interface, RelayNodeEnum)]
#[graphql(field(name = "id", type = "String"))]
pub enum Node {
    User(User),
    Tenant(Tenant),
}

#[Object]
impl QueryRoot {
    async fn user(&self) -> User {
        User {
            id: ID(
                "92ba0c2d-4b4e-4e29-91dd-8f96a078c3ff".to_string(),
                SchemaNodeTypes::User,
            ),
            name: "Oscar".to_string(),
            role: "Testing123".to_string(),
        }
    }

    async fn tenant(&self) -> Tenant {
        Tenant {
            id: ID(
                "14b4a5db-b8f0-4bf9-881e-37a9e0d0ae3h".to_string(),
                SchemaNodeTypes::Tenant,
            ),
            name: "My Company".to_string(),
            description: "Testing123".to_string(),
        }
    }

    async fn node(&self, id: String) -> Option<Node> {
        Node::get(id).await
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn handler(schema: web::Data<Schema>, req: Request) -> impl Responder {
    let res: Response = schema.execute(req.into_inner()).await;

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .json(res)
}

pub async fn playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Listening...");
    HttpServer::new(move || {
        App::new().data(schema.clone()).service(
            web::resource("/graphql")
                .route(web::get().to(playground))
                .route(web::post().to(handler)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
