pub trait EntityStorage<T>: Sync + Send where T: IdentifiedEntity{
    fn add(&self, entity: T) -> T;
    fn remove(&self, id: &str);
    fn get_by_id(&self, id: &str) -> Option<T>;
    fn get_all(&self) -> Vec<T>;
}


//TODO(pfz4): Consider moving this to Domain Layer
pub trait IdentifiedEntity: Sync+Send+Clone{
    fn id(&self) -> String;
}

impl IdentifiedEntity for domain::artist::Artist {
    fn id(&self) -> String {
        self.name.clone()
    }
}

impl IdentifiedEntity for domain::song::Song {
    fn id(&self) -> String {
        self.name.clone()
    }
}