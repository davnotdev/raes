use parking_lot::{RwLock, RwLockWriteGuard};

#[derive(Default)]
pub struct CopySwap<T: Copy> {
    datas: [RwLock<T>; 2],
    read_index: usize,
}

impl<T: Copy> CopySwap<T> {
    pub fn new(init_val: T) -> Self {
        Self {
            datas: [RwLock::new(init_val), RwLock::new(init_val)],
            read_index: 0,
        }
    }

    pub fn swap(&mut self) {
        let write_index = self.get_write_index();
        *self.datas[self.read_index].write() = *self.datas[write_index].read();
        self.read_index = write_index;
    }

    fn get_write_index(&self) -> usize {
        (self.read_index + 1) % 2
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.datas[self.read_index].data_ptr() }
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<T> {
        self.datas[self.get_write_index()].write()
    }
}
