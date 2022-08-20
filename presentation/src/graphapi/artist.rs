use juniper::{graphql_object, FieldResult};

use crate::graphapi::song::Song;

use domain::artist::Artist as DomainArtist;
use domain::song::Song as DomainSong;

use super::context::Context;

/// Artist DTO for GraphQL
pub(crate) struct Artist {
    /// Name of the Artist
    pub(crate) name: String
}

/// GraphQL definition for Artist DTO
#[graphql_object(context=Context)]
impl Artist {
    ///Name of the Artist
    fn name(&self) -> String {
        self.name.clone()
    }

    ///All Songs that the Artist created
    fn songs(&self, executor: &Executor<Context>) -> FieldResult<Vec<Song>> {
        let domain_songs: Vec<DomainSong> = executor.context().song_storage.get_all();
        let songs = domain_songs.iter()
            .filter(|x|x.artist_name==self.name)
            .map(|x|Song::from(x.clone()))
            .collect::<Vec<Song>>();
        Ok(songs)
    }
}

/// Translation from Domain Artist to GraphQL Artist DTO
impl From<DomainArtist> for Artist{
    fn from(artist: DomainArtist) -> Self {
        Self {
            name: artist.name
        }
    }
}