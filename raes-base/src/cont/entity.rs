use super::*;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Entity<C: Cont> {
    id: usize,
    generation: usize,
    _phantom: PhantomData<C>,
}

impl<C: Cont> Entity<C> {
    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, Copy)]
struct Generation(usize);
#[derive(Debug, Clone, Copy)]
struct Exists(bool);

#[derive(Debug, Clone, Copy)]
pub struct ContEntities<C: Cont, const MAX_ENTITIES: usize = DEFAULT_MAX_ENTITIES> {
    entities: [(Generation, Exists); MAX_ENTITIES],
    _phantom: PhantomData<C>,
}

impl<C: Cont, const MAX_ENTITIES: usize> ContEntities<C, MAX_ENTITIES> {
    pub fn new() -> Self {
        Self {
            entities: [(Generation(0), Exists(false)); MAX_ENTITIES],
            _phantom: PhantomData,
        }
    }

    pub fn spawn(&mut self) -> Entity<C> {
        self.entities
            .iter_mut()
            .enumerate()
            .find_map(|(id, (Generation(generation), Exists(exists)))| {
                (!*exists).then(|| {
                    *exists = true;
                    Entity {
                        id,
                        generation: *generation,
                        _phantom: PhantomData,
                    }
                })
            })
            .unwrap_or_else(|| {
                panic!(
                    "On spawn: exceeded max entities of {} for container {}.",
                    MAX_ENTITIES,
                    std::any::type_name::<C>()
                );
            })
    }

    pub fn despawn(&mut self, entity: Entity<C>) -> Option<()> {
        if self.has_entity(&entity) {
            None?
        }

        let (Generation(generation), Exists(exists)) = self.entities.get_mut(entity.id()).unwrap();
        *generation += 1;
        *exists = false;

        Some(())
    }

    pub fn has_entity(&mut self, entity: &Entity<C>) -> bool {
        self.entities
            .get(entity.id())
            .map(|&(Generation(generation), Exists(exists))| {
                generation == entity.generation && exists
            })
            .unwrap_or(false)
    }
}
