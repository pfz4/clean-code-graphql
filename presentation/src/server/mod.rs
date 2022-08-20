use std::sync::Arc;

use juniper::{RootNode, EmptySubscription};
use rocket::routes;

use infrastructure::memory_storage::MemoryStorage;
use domain::song::Song as DomainSong;
use domain::artist::Artist as DomainArtist;

use crate::graphapi::{query::Query, mutation::Mutation, context::Context};
use infrastructure::demo_authentication_provider::DemoAuthenticationProvider;
use application::entity_storage::EntityStorage;
use application::authentication_provider::AuthenticationProvider;

mod routes;
mod guards;

pub(crate) type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub async fn launch() -> Result<rocket::Rocket<rocket::Ignite>, rocket::Error>{
    //Create Song EntityStorage with MemoryStorage
    let song_storage: Arc<Box<dyn EntityStorage<DomainSong>>> = Arc::new(Box::new(MemoryStorage::<DomainSong>::default()));

    //Create Artist EntityStorage with MemoryStorage
    let artist_storage: Arc<Box<dyn EntityStorage<DomainArtist>>> = Arc::new(Box::new(MemoryStorage::<DomainArtist>::default()));

    //Create Authentication Provider with DemoAuthenticationProvider
    let authentication_provider: Box<dyn AuthenticationProvider> = Box::new(DemoAuthenticationProvider::default());
    
    rocket::build()
        .manage(Schema::new(Query, Mutation, EmptySubscription::new()))
        .manage(song_storage)
        .manage(artist_storage)
        .manage(authentication_provider)
        .mount("/", routes![
            routes::graphiql,
            routes::graphql_handler
        ])
        .launch()
        .await
}