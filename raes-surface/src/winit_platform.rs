use super::*;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use winit::{
    event::{ElementState, Event, MouseButton as WindowMouseButton, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub struct WindowCont {
    window: Window,
    event_loop: EventLoop<()>,
    surface_edge: Manual<Arc<CopySwap<SurfaceEdgeData>>>,
    window_closed: Arc<RwLock<bool>>,
}

impl WindowCont {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        window.set_maximized(true);
        WindowCont {
            window,
            event_loop,
            surface_edge: Manual::default(),
            window_closed: Arc::new(RwLock::new(false)),
        }
    }
}

impl Surface for WindowCont {
    fn set_window_edge(&mut self, edge: &Arc<CopySwap<SurfaceEdgeData>>) {
        self.surface_edge.init(Arc::clone(edge));
    }

    fn get_raw_display(&self) -> RawDisplayHandle {
        self.event_loop.raw_display_handle()
    }

    fn get_raw_window(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }

    fn get_width(&self) -> usize {
        self.window.inner_size().width as usize
    }

    fn get_height(&self) -> usize {
        self.window.inner_size().height as usize
    }

    fn window_closed(&self) -> bool {
        *self.window_closed.read()
    }

    fn update(&mut self) {
        let window_closed = Arc::clone(&self.window_closed);
        let surface_edge = Arc::clone(&self.surface_edge);

        self.event_loop.run_return(move |event, _, control_flow| {
            let surface_edge = &surface_edge;
            let window_closed = &window_closed;

            let mut edge = surface_edge.get_mut();
            edge.mouse_scroll = MouseScrollState {
                delta_x: 0.0,
                delta_y: 0.0,
            };
            edge.mouse_position = MousePositionState {
                delta_x: 0.0,
                delta_y: 0.0,
                ..edge.mouse_position
            };

            control_flow.set_poll();
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *window_closed.write() = true;
                    }
                    WindowEvent::Resized(resize) => edge.resize_events.push(ResizeEvent {
                        width: resize.width as usize,
                        height: resize.height as usize,
                    }),
                    WindowEvent::ModifiersChanged(state) => {
                        edge.modifers.alt = state.alt();
                        edge.modifers.ctrl = state.ctrl();
                        edge.modifers.logo = state.logo();
                        edge.modifers.shift = state.shift();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let delta_x = edge.mouse_position.x - position.x as f32;
                        let delta_y = edge.mouse_position.y - position.y as f32;

                        edge.mouse_position = MousePositionState {
                            x: position.x as f32,
                            y: position.y as f32,
                            delta_x,
                            delta_y,
                        };
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let delta = match delta {
                            MouseScrollDelta::LineDelta(dx, dy) => MouseScrollState {
                                delta_x: dx,
                                delta_y: dy,
                            },
                            MouseScrollDelta::PixelDelta(pd) => MouseScrollState {
                                delta_x: pd.x as f32,
                                delta_y: pd.y as f32,
                            },
                        };
                        edge.mouse_scroll = delta;
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        let press = match state {
                            ElementState::Pressed => PressState::Down,
                            ElementState::Released => PressState::Up,
                        };
                        if let Some(button) = match button {
                            WindowMouseButton::Left => Some(MouseButton::Left),
                            WindowMouseButton::Right => Some(MouseButton::Middle),
                            WindowMouseButton::Middle => Some(MouseButton::Right),
                            _ => None,
                        } {
                            edge.click_events.push(ClickEvent { press, button })
                        }
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        let press = match input.state {
                            ElementState::Pressed => PressState::Down,
                            ElementState::Released => PressState::Up,
                        };
                        if let Some(virtual_keycode) = input.virtual_keycode {
                            let keycode = unsafe { std::mem::transmute(virtual_keycode as u32) };
                            edge.key_events.push(KeyEvent { press, keycode })
                        };
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    control_flow.set_exit();
                }
                _ => {}
            };
        });
    }
}
