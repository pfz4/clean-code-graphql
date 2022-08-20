use juniper::GraphQLEnum;
use domain::genre::Genre as DomainGenre;

/// Genre DTO for GraphQL
/// Genre of a Song
#[derive(GraphQLEnum, Debug, Clone, Copy)]
pub(crate) enum Genre{
    /// Pop Genre
    Pop,

    /// Rock Genre
    Rock,

    /// Electro Genre
    Electro,

    /// Swing Genre
    Swing
}

/// Translation from Domain Genre to GraphQL Genre DTO
impl From<DomainGenre> for Genre {
    fn from(genre: DomainGenre) -> Self {
        match genre {
            DomainGenre::Pop => Self::Pop,
            DomainGenre::Rock => Self::Rock,
            DomainGenre::Electro => Self::Electro,
            DomainGenre::Swing => Self::Swing
        }
    }
}


/// Translation from GraphQL Gentre DTO to Domain Genre
impl Into<DomainGenre> for Genre {
    fn into(self) -> DomainGenre {
        match self{
            Genre::Pop => DomainGenre::Pop,
            Genre::Rock => DomainGenre::Rock,
            Genre::Electro => DomainGenre::Electro,
            Genre::Swing => DomainGenre::Swing
        }
    }
}