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

    let mut state = render::State::new_async(window).await;
    let mut game_data = game::GameData::new();

    game_data.blocks[0][0] = Some(game::BlockColor::Red);
    game_data.blocks[0][1] = Some(game::BlockColor::Green);
    game_data.blocks[0][2] = Some(game::BlockColor::Blue);
    game_data.blocks[19][9] = Some(game::BlockColor::Orange);
    game_data.blocks[19][9] = Some(game::BlockColor::Purple);

    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id,
            ref event,
        } if state.match_id(window_id) => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(new_size) => {
                state.resize(*new_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if state.match_id(window_id) => {
            state.render(&game_data);
        }
        Event::RedrawEventsCleared => {
            state.request_redraw();
        }
        _ => {}
    });
}
