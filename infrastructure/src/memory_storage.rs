use std::sync::{Arc, Mutex};

use application::entity_storage::{EntityStorage, IdentifiedEntity};

pub struct MemoryStorage<T: IdentifiedEntity> where T:IdentifiedEntity {
    entities: Arc<Mutex<Vec<T>>>
}

impl<T> Default for MemoryStorage<T> where T: IdentifiedEntity{
    fn default() -> Self {
        Self { entities: Default::default() }
    }
}

impl<T> EntityStorage<T> for MemoryStorage<T> where T: IdentifiedEntity{
    fn add(&self, entity: T) -> T{
        let mut entities = self.entities.lock().unwrap();
        entities.push(entity.clone());
        entity
    }

    fn remove(&self, id: &str) {
        let mut entities = self.entities.lock().unwrap();
        let position = entities.iter().position(|x|x.id()==id);
        if let Some(position) = position {
            entities.remove(position);
        }
    }

    fn get_by_id(&self, id: &str) -> Option<T> {
        let entities = self.entities.lock().unwrap();
        entities.iter()
            .find(|x|x.id()==id)
            .map(|x|x.clone())
    }

    fn get_all(&self) -> Vec<T> {
        let entities = self.entities.lock().unwrap();
        entities.clone()
    }
}