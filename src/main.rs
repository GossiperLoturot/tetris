use std::sync::Arc;

mod consts;
mod game;
mod render;

pub struct State {
    game_system: game::GameSystem,
    render_system: render::RenderSystem,
}

impl State {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let game_system = game::GameSystem::Start(game::start::GameSystem::new());
        let render_system = pollster::block_on(render::RenderSystem::new_async(window));

        Self {
            game_system,
            render_system,
        }
    }
}

pub struct App {
    state: Option<State>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attrs = winit::window::WindowAttributes::default();
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        self.state = Some(State::new(window));
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        if cause == winit::event::StartCause::Poll {
            state.game_system.update();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        if !state.render_system.match_id(window_id) {
            return;
        }

        match event {
            winit::event::WindowEvent::RedrawRequested => {
                state.render_system.render(state.game_system.context());
                state.render_system.request_redraw();
            }
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(new_size) => {
                state.render_system.resize(new_size);
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                state.game_system.input(&event);
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
