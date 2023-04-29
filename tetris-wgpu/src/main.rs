mod game;
mod render;

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut game_system = game::GameSystem::Start(game::start::GameSystem::new());
    let mut render_system = render::RenderSystem::new_async(window).await;

    use winit::event::Event;
    use winit::event::StartCause;
    use winit::event::WindowEvent;
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(StartCause::Poll) => {
            game_system.update();
        }
        Event::WindowEvent {
            window_id,
            ref event,
        } if render_system.match_id(window_id) => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                game_system.input(input);
            }
            WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            WindowEvent::Resized(new_size) => {
                render_system.resize(*new_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                render_system.resize(**new_inner_size);
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if render_system.match_id(window_id) => {
            render_system.render(game_system.context());
        }
        Event::RedrawEventsCleared => {
            render_system.request_redraw();
        }
        _ => {}
    });
}
