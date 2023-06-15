#[derive(Debug, Clone, Copy)]
pub struct EventBuffer<T, const MAX: usize> {
    events: [T; MAX],
    events_count: usize,
}

impl<T, const MAX: usize> EventBuffer<T, MAX> {
    pub fn push(&mut self, event: T) {
        let idx = self.events_count + 1;
        if idx < MAX {
            self.events[idx] = event;
        }
    }

    pub fn flush(&mut self) {
        self.events_count = 0;
    }

    pub fn as_slice(&self) -> &[T] {
        &self.events[0..self.events_count]
    }

    pub fn iter(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const MAX: usize> EventBuffer<T, MAX>
where
    T: Clone,
{
    /// `init_val` will never be read.
    /// Rather, the value is just used to safely initialize the buffer.
    pub fn new(init_val: T) -> Self {
        let events = unsafe {
            (0..MAX)
                .map(|_| init_val.clone())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_unchecked()
        };
        Self {
            events,
            events_count: 0,
        }
    }
}

impl<T, const MAX: usize> EventBuffer<T, MAX>
where
    T: Default,
{
    pub fn new_with_default() -> Self {
        let events = unsafe {
            (0..MAX)
                .map(|_| T::default())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_unchecked()
        };
        Self {
            events,
            events_count: 0,
        }
    }
}

impl<T, const MAX: usize> Default for EventBuffer<T, MAX>
where
    T: Default,
{
    fn default() -> Self {
        Self::new_with_default()
    }
}
