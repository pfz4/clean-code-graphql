use application::{song_service, artist_service};
use juniper::{graphql_object, FieldResult, FieldError, Value};

use crate::graphapi::artist::Artist;
use crate::graphapi::genre::Genre;
use crate::graphapi::song::Song;

use super::context::Context;

/// GraphQL Root Mutation
pub(crate) struct Mutation;

#[graphql_object(context=Context)]
impl Mutation {
    ///Create a Song
    fn create_song(executor: &Executor<context::Context>, name: String, artist: String, genre: Genre) -> FieldResult<Song> {
        let context = executor.context();
        match &context.user {
            Some(user) => song_service::create_song_auth(name, artist, genre.into(), &context.song_storage, user)
                .map(|x|x.into())
                .map_err(|_|FieldError::new("Forbidden", Value::null())),
            None => Err(FieldError::new("Unauthorized", Value::null()))
        }
    }

    /// Delete a Song
    fn delete_song(executor: &Executor<context::Context>, name: String) -> FieldResult<String> {
        let context = executor.context();
        match &context.user {
            Some(user) => song_service::delete_song_auth(&name, &context.song_storage, user)
                .map(|_|name)
                .map_err(|_|FieldError::new("Forbidden", Value::null())),
            None => Err(FieldError::new("Unauthorized", Value::null()))
        }
    }

    /// Create an Artist
    fn create_artist(executor: &Executor<Context>, name: String) -> FieldResult<Artist> {
        let context = executor.context();
        match &context.user {
            Some(user) => artist_service::create_artist_auth(name, &context.artist_storage, user)
                .map(|x|x.into())
                .map_err(|_|FieldError::new("Forbidden", Value::null())),
            None => Err(FieldError::new("Unauthorized", Value::null()))
        }
    }

    /// Delete an Artist
    fn delete_artist(executor: &Executor<Context>, name: String) -> FieldResult<String> {
        let context = executor.context();
        match &context.user {
            Some(user) => artist_service::delete_artist_auth(&name, &context.artist_storage, user)
                .map(|_|name)
                .map_err(|_|FieldError::new("Forbidden", Value::null())),
            None => Err(FieldError::new("Unauthorized", Value::null()))
        }
    }
}