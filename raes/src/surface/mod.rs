use crate::base::*;

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
    fn surface_closed(&self) -> bool;
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

    pub async fn wait_surface_closed(&self) {
        loop {
            if self.surface_closed() {
                return;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
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

    pub fn surface_closed(&self) -> bool {
        self.surface.surface_closed()
    }

    pub fn update(&mut self) {
        self.surface.update()
    }
}

impl Preservable for SurfaceCont {}

const MAX_KEY_EVENT_COUNT: usize = 16;
const MAX_CLICK_EVENT_COUNT: usize = 4;
const MAX_RESIZE_EVENT_COUNT: usize = 2;

#[derive(Debug, Clone, Copy)]
pub struct SurfaceEdgeData {
    modifers: ModifierState,
    mouse_scroll: MouseScrollState,
    mouse_position: MousePositionState,
    key_events: EventBuffer<KeyEvent, MAX_KEY_EVENT_COUNT>,
    click_events: EventBuffer<ClickEvent, MAX_CLICK_EVENT_COUNT>,
    resize_events: EventBuffer<ResizeEvent, MAX_RESIZE_EVENT_COUNT>,
}

impl SurfaceEdgeData {
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
