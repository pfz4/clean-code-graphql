#[derive(Debug, PartialEq)]
pub enum Permission {
    /// The User is allowed to Create a new Song
    CreateSong,

    /// The User is allowed to Delete a existing Song
    DeleteSong,

    /// The User is allowed to Create a new Artist
    CreateArtist,

    /// The User is allowed to Delete an extisting Artist
    DeleteArtist
}