use juniper::{graphql_object, FieldResult};

use domain::song::Song as DomainSong;
use domain::artist::Artist as DomainArtist;
use crate::graphapi::artist::Artist;
use crate::graphapi::song::Song;

use super::context::Context;

/// GraphQL Root Query
pub(crate) struct Query;

#[graphql_object(context=Context)]
impl Query {
    /// Get all Songs
    pub(crate) fn songs(executor: &Executor<Context>) -> FieldResult<Vec<Song>> {
        //TODO(pfz4): Move to application Layer
        let domain_songs: Vec<DomainSong> = executor.context().song_storage.get_all();
        let songs = domain_songs.iter()
            .map(|x|Song::from(x.clone()))
            .collect::<Vec<Song>>();
        Ok(songs)
    }

    /// Get all Artists
    pub(crate) fn artists(executor: &Executor<Context>) -> FieldResult<Vec<Artist>> {
        //TODO(pfz4): Move to application Layer
        let domain_artists: Vec<DomainArtist> = executor.context().artist_storage.get_all();
        let artists = domain_artists.iter()
            .map(|x|Artist::from(x.clone()))
            .collect::<Vec<Artist>>();
        Ok(artists)
    }
}
