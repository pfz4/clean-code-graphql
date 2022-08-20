use domain::authentication::permission::Permission;
use domain::authentication::user::User;
use domain::genre::Genre;
use domain::song::Song;

use crate::entity_storage::EntityStorage;

type ServiceResult<T> = Result<T, ()>;

pub fn create_song_auth(name: String, artist: String, genre: Genre, song_storage: &Box<dyn EntityStorage<Song>>, user: &User) -> ServiceResult<Song> {
    if !user.permissions.contains(&Permission::CreateSong) {
        return Err(()); // Forbidden
    }
    create_song(name, artist, genre, song_storage)
}

fn create_song(name: String, artist_name: String, genre: Genre, song_storage: &Box<dyn EntityStorage<Song>>) -> ServiceResult<Song> {
    let entity = Song{
        name,
        artist_name,
        genre
    };

    Ok(song_storage.add(entity))
}


pub fn delete_song_auth(name: &str, song_storage: &Box<dyn EntityStorage<Song>>, user: &User) -> ServiceResult<()> {
    if !user.permissions.contains(&Permission::DeleteArtist) {
        return Err(()); // Forbidden
    }
    delete_song(name, song_storage)
}

fn delete_song(name: &str, song_storage: &Box<dyn EntityStorage<Song>>) -> ServiceResult<()> {
    song_storage.remove(name);
    Ok(())
}