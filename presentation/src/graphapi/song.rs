use juniper::{graphql_object, FieldResult, FieldError, Value};

use crate::graphapi::artist::Artist;
use domain::song::Song as DomainSong;
use domain::artist::Artist as DomainArtist;
use super::context::Context;
use super::genre::Genre;

/// Song DTO for GraphQL
pub(crate) struct Song {
    /// Name of the Song
    pub(crate) name: String,

    /// Genre of the Song
    pub(crate) genre: Genre,

    /// Artist of the Song
    pub(crate) artist_name: String,
}

/// GraphQL definition for Song DTO
#[graphql_object(context=Context)]
impl Song {
    /// Name of the Song
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Artist / Creator of the Song
    fn artist(&self, executor: &Executor<Context>) -> FieldResult<Artist> {
        let artist: Option<DomainArtist> = executor.context().artist_storage.get_by_id(&self.artist_name);
        artist.map(|x|Artist::from(x))
            .ok_or(FieldError::new("Artist not found", Value::null()))
    }

    /// Genre of the Song
    fn genre(&self) -> Genre {
        self.genre
    }
}

/// Translation from Domain Song to GraphQL Song DTO
impl From<DomainSong> for Song{
    fn from(song: DomainSong) -> Self {
        Self {
            name: song.name,
            genre: song.genre.into(),
            artist_name: song.artist_name   
        }
    }
}