pub mod end;
pub mod playing;
pub mod start;

#[derive(Debug)]
pub enum GameEvent {
    Play,
    End(i32),
}

pub enum GameContext<'a> {
    Start,
    Playing(playing::GameContext<'a>),
    End(end::GameContext<'a>),
}

pub enum GameSystem {
    Start(start::GameSystem),
    Playing(playing::GameSystem),
    End(end::GameSystem),
}

impl GameSystem {
    pub fn new() -> Self {
        Self::Start(start::GameSystem::new())
    }

    pub fn input(
        &mut self,
        input: &winit::event::KeyboardInput,
        event_sender: &winit::event_loop::EventLoopProxy<GameEvent>,
    ) {
        match self {
            GameSystem::Start(system) => {
                system.input(input, event_sender);
            }
            GameSystem::Playing(system) => {
                system.input(input);
            }
            GameSystem::End(_) => {
                // nothing
            }
        }
    }

    pub fn update(&mut self, event_sender: &winit::event_loop::EventLoopProxy<GameEvent>) {
        match self {
            GameSystem::Start(_) => {
                // nothing
            }
            GameSystem::Playing(system) => {
                system.update(event_sender);
            }
            GameSystem::End(_) => {
                // nothing
            }
        }
    }

    pub fn receive_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::Play => {
                *self = GameSystem::Playing(playing::GameSystem::new());
            }
            GameEvent::End(score) => {
                *self = GameSystem::End(end::GameSystem::new(score));
            }
        }
    }

    pub fn context(&self) -> GameContext {
        match self {
            GameSystem::Start(_) => GameContext::Start,
            GameSystem::Playing(system) => GameContext::Playing(system.context()),
            GameSystem::End(system) => GameContext::End(system.context()),
        }
    }
}
