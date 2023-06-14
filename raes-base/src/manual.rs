use std::ops::{Deref, DerefMut};

pub struct Manual<T> {
    data: Option<T>,
}

impl<T> Default for Manual<T> {
    fn default() -> Self {
        Self { data: None }
    }
}

impl<T> Manual<T> {
    pub fn init(&mut self, data: T) {
        if self.data.is_some() {
            panic!("Manual<{}> initialized twice.", std::any::type_name::<T>())
        }

        self.data = Some(data);
    }

    pub fn take(&mut self) -> T {
        if self.data.is_none() {
            panic!("Manual<{}> nothing to take.", std::any::type_name::<T>())
        }

        std::mem::replace(&mut self.data, None).unwrap()
    }
}

impl<T> Deref for Manual<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.data.is_none() {
            panic!(
                "&Manual<{}> is not initialized.",
                std::any::type_name::<T>()
            )
        }

        self.data.as_ref().unwrap()
    }
}

impl<T> DerefMut for Manual<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_none() {
            panic!(
                "&mut Manual<{}> is not initialized.",
                std::any::type_name::<T>()
            )
        }

        self.data.as_mut().unwrap()
    }
}
