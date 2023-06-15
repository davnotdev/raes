use parking_lot::{Mutex, MutexGuard};

pub trait Flushable {
    fn flush(&mut self) {}
}

#[derive(Default)]
pub struct CopySwap<T: Copy + Flushable> {
    datas: [Mutex<T>; 2],
    read_index: usize,
}

impl<T: Copy + Flushable> CopySwap<T> {
    pub fn new(init_val: T) -> Self {
        Self {
            datas: [Mutex::new(init_val), Mutex::new(init_val)],
            read_index: 0,
        }
    }

    pub fn flush(&mut self) {
        let write_index = self.get_write_index();
        let mut write = self.datas[write_index].lock();
        write.flush();
        *self.datas[self.read_index].lock() = *write;
        self.read_index = write_index;
    }

    fn get_write_index(&self) -> usize {
        (self.read_index + 1) % 2
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.datas[self.read_index].data_ptr() }
    }

    pub fn get_mut(&self) -> MutexGuard<T> {
        self.datas[self.get_write_index()].lock()
    }
}
