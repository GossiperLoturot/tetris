mod game;
mod render;

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut cx = game::GameContext::new();
    let mut render_system = render::RenderSystem::new_async(window).await;

    cx.blocks[0][0] = Some(game::BlockColor::Red);
    cx.blocks[0][1] = Some(game::BlockColor::Green);
    cx.blocks[0][2] = Some(game::BlockColor::Blue);
    cx.blocks[19][9] = Some(game::BlockColor::Orange);
    cx.blocks[19][9] = Some(game::BlockColor::Purple);

    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id,
            ref event,
        } if render_system.match_id(window_id) => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(new_size) => {
                render_system.resize(*new_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                render_system.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if render_system.match_id(window_id) => {
            render_system.render(&cx);
        }
        Event::RedrawEventsCleared => {
            render_system.request_redraw();
        }
        _ => {}
    });
}
