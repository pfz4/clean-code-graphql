use super::genre::Genre;

#[derive(Debug, Clone)]
pub  struct Song {
    pub name: String,
    pub genre: Genre,
    pub artist_name: String,
}