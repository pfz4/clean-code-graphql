use std::sync::Arc;

use application::entity_storage::EntityStorage;
use domain::authentication::user::User;
use domain::song::Song as DomainSong;
use domain::artist::Artist as DomainArtist;

pub(crate) struct Context{
    /// Song persistence Provider
    pub(crate) song_storage: Arc<Box<dyn EntityStorage<DomainSong>>>, 

    /// Artist persistence Provider
    pub(crate) artist_storage: Arc<Box<dyn EntityStorage<DomainArtist>>>,

    /// User information
    pub(crate) user: Option<User>
}

impl juniper::Context for Context{}