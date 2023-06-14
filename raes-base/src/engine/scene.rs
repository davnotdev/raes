use std::{
    any::{type_name, TypeId},
    collections::HashMap,
};

pub enum SceneExit {
    Next(String, IceBox),
    End,
}

pub trait Scene {
    fn run(&mut self, icebox: IceBox) -> Result<SceneExit, String>;
}

pub trait Preservable {}

#[derive(Default)]
pub struct IceBox {
    preserved: HashMap<TypeId, Box<dyn Preservable>>,
}

impl IceBox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take<P: Preservable + 'static>(&mut self) -> Option<&mut P> {
        let id = TypeId::of::<P>();
        let p = self.preserved.remove(&id)?;
        Some(unsafe { &mut *(Box::leak(p) as *mut dyn Preservable as *mut P) })
    }

    pub fn put<P: Preservable + 'static>(&mut self, data: P) {
        let id = TypeId::of::<P>();
        if self.preserved.insert(id, Box::new(data)).is_some() {
            panic!("Multiple `{}`s put into icebox.", type_name::<P>())
        }
    }
}
