use std::sync::Arc;

use application::authentication_provider::AuthenticationProvider;
use application::entity_storage::EntityStorage;
use rocket::{get, post, response::content, State};

use crate::graphapi::context::Context;
use crate::server::Schema;
use crate::server::guards::BearerToken;

///GraphQL UI Playground
#[get("/")]
pub(crate) fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

///GraphQL Endpoint
#[post("/graphql", data = "<request>")]
pub(crate) async fn graphql_handler(
    request: juniper_rocket::GraphQLRequest, 
    schema: &State<Schema>, 
    song_storage: &State<Arc<Box<dyn EntityStorage<domain::song::Song>>>>,
    artist_storage: &State<Arc<Box<dyn EntityStorage<domain::artist::Artist>>>>,
    authentication_provider: &State<Box<dyn AuthenticationProvider>>,
    bearer_token: Option<BearerToken>
) -> juniper_rocket::GraphQLResponse{
    let user = match bearer_token {
        Some(bearer_token) => authentication_provider.from_token(&bearer_token.0),
        None => None
    };
    let context = Context{
        song_storage: song_storage.inner().clone(),
        artist_storage: artist_storage.inner().clone(),
        user
    };
    
    request.execute(&*schema, &context)
        .await
}