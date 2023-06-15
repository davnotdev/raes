use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Row<T, const MAX_ENTITIES: usize = DEFAULT_MAX_ENTITIES> {
    datas: [T; MAX_ENTITIES],
}

impl<T, const MAX_ENTITIES: usize> Row<T, MAX_ENTITIES> {
    pub fn get<C: Cont>(&self, entity: Entity<C>) -> &T {
        self.datas.get(entity.id()).unwrap_or_else(|| {
            panic!(
                "On get: exceeded max entities of {} for container {}, row type {}.",
                MAX_ENTITIES,
                std::any::type_name::<C>(),
                std::any::type_name::<T>()
            );
        })
    }

    pub fn get_mut<C: Cont>(&mut self, entity: Entity<C>) -> &mut T {
        self.datas.get_mut(entity.id()).unwrap_or_else(|| {
            panic!(
                "On get_mut: exceeded max entities of {} for container {}, row type {}.",
                MAX_ENTITIES,
                std::any::type_name::<C>(),
                std::any::type_name::<T>()
            );
        })
    }

    pub fn as_slice(&self) -> &[T] {
        &self.datas
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.datas
    }
}

impl<T, const MAX_ENTITIES: usize> Row<T, MAX_ENTITIES>
where
    T: Clone,
{
    pub fn new(init_val: T) -> Self {
        let datas = unsafe {
            (0..MAX_ENTITIES)
                .map(|_| init_val.clone())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_unchecked()
        };
        Self { datas }
    }
}

impl<T, const MAX_ENTITIES: usize> Row<T, MAX_ENTITIES>
where
    T: Default,
{
    pub fn new_with_default() -> Self {
        let datas = unsafe {
            (0..MAX_ENTITIES)
                .map(|_| T::default())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_unchecked()
        };
        Self { datas }
    }
}

impl<T, const MAX_ENTITIES: usize> Default for Row<T, MAX_ENTITIES>
where
    T: Default,
{
    fn default() -> Self {
        Self::new_with_default()
    }
}
