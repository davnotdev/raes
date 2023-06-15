use crate::base::*;
use std::sync::Arc;

mod event;
mod winit_platform;

pub use event::{
    ClickEvent, KeyEvent, ModifierState, MouseButton, MousePositionState, MouseScrollState,
    PressState, ResizeEvent, VirtualKeyCode,
};
pub use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub trait Surface {
    fn set_window_edge(&mut self, edge: &Arc<CopySwap<SurfaceEdgeData>>);

    fn get_raw_display(&self) -> RawDisplayHandle;
    fn get_raw_window(&self) -> RawWindowHandle;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn window_closed(&self) -> bool;
    fn update(&mut self);
}

pub struct SurfaceCont {
    surface: Box<dyn Surface>,
}

impl SurfaceCont {
    pub fn new() -> Self {
        SurfaceCont {
            surface: Box::new(winit_platform::WindowCont::new()),
        }
    }

    pub fn get_raw_display(&self) -> RawDisplayHandle {
        self.surface.get_raw_display()
    }

    pub fn get_raw_window(&self) -> RawWindowHandle {
        self.surface.get_raw_window()
    }

    pub fn get_width(&self) -> usize {
        self.surface.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.surface.get_height()
    }

    pub fn window_closed(&self) -> bool {
        self.surface.window_closed()
    }

    pub fn update(&mut self) {
        self.surface.update()
    }
}

impl Preservable for SurfaceCont {}

const DEFAULT_KEY_EVENT_BUFFER_COUNT: usize = 16;
const DEFAULT_CLICK_EVENT_BUFFER_COUNT: usize = 4;
const DEFAULT_RESIZE_EVENT_BUFFER_COUNT: usize = 2;

#[derive(Debug, Clone, Copy)]
pub struct SurfaceEdgeData<
    const KEY_EVENT_BUFFER_COUNT: usize = DEFAULT_KEY_EVENT_BUFFER_COUNT,
    const MOUSE_EVENT_BUFFER_COUNT: usize = DEFAULT_CLICK_EVENT_BUFFER_COUNT,
    const RESIZE_EVENT_BUFFER_COUNT: usize = DEFAULT_RESIZE_EVENT_BUFFER_COUNT,
> {
    modifers: ModifierState,
    mouse_scroll: MouseScrollState,
    mouse_position: MousePositionState,
    key_events: EventBuffer<KeyEvent, KEY_EVENT_BUFFER_COUNT>,
    click_events: EventBuffer<ClickEvent, MOUSE_EVENT_BUFFER_COUNT>,
    resize_events: EventBuffer<ResizeEvent, RESIZE_EVENT_BUFFER_COUNT>,
}

impl<
        const KEY_EVENT_BUFFER_COUNT: usize,
        const MOUSE_EVENT_BUFFER_COUNT: usize,
        const RESIZE_EVENT_BUFFER_COUNT: usize,
    > SurfaceEdgeData<KEY_EVENT_BUFFER_COUNT, MOUSE_EVENT_BUFFER_COUNT, RESIZE_EVENT_BUFFER_COUNT>
{
    pub fn new() -> Self {
        Self {
            modifers: ModifierState::default(),
            mouse_scroll: MouseScrollState::default(),
            mouse_position: MousePositionState::default(),
            key_events: EventBuffer::new(KeyEvent {
                press: PressState::Up,
                keycode: VirtualKeyCode::Q,
            }),
            click_events: EventBuffer::new(ClickEvent {
                press: PressState::Up,
                button: MouseButton::Left,
            }),
            resize_events: EventBuffer::new(ResizeEvent {
                width: 0,
                height: 0,
            }),
        }
    }
}

impl Flushable for SurfaceEdgeData {
    fn flush(&mut self) {
        self.key_events.flush();
        self.click_events.flush();
        self.resize_events.flush();
    }
}
