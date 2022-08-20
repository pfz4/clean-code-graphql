use domain::authentication::permission::Permission;
use domain::authentication::user::User;
use domain::artist::Artist;

use crate::entity_storage::EntityStorage;

type ServiceResult<T> = Result<T, ()>;

pub fn create_artist_auth(name: String, artist_storage: &Box<dyn EntityStorage<Artist>>, user: &User) -> ServiceResult<Artist> {
    if !user.permissions.contains(&Permission::CreateArtist) {
        return Err(()); // Forbidden
    }
    create_artist(name, artist_storage)
}

fn create_artist(name: String, artist_storage: &Box<dyn EntityStorage<Artist>>) -> ServiceResult<Artist> {
    let entity = Artist{
        name,
    };

    Ok(artist_storage.add(entity))
}


pub fn delete_artist_auth(name: &str, artist_storage: &Box<dyn EntityStorage<Artist>>, user: &User) -> ServiceResult<()> {
    if !user.permissions.contains(&Permission::DeleteArtist) {
        return Err(()); // Forbidden
    }
    delete_artist(name, artist_storage)
}

fn delete_artist(name: &str, artist_storage: &Box<dyn EntityStorage<Artist>>) -> ServiceResult<()> {
    artist_storage.remove(name);
    Ok(())
}