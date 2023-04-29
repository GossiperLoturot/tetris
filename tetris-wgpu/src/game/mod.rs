pub mod end;
pub mod playing;
pub mod start;

pub enum GameContext<'a> {
    Start(start::GameContext),
    Playing(playing::GameContext<'a>),
    End(end::GameContext<'a>),
}

pub enum GameSystem {
    Start(start::GameSystem),
    Playing(playing::GameSystem),
    End(end::GameSystem),
}

impl GameSystem {
    pub fn input(&mut self, input: &winit::event::KeyboardInput) {
        let mut flow = GameSystemFlow::Default;
        match self {
            GameSystem::Start(system) => system.input(input, &mut flow),
            GameSystem::Playing(system) => system.input(input, &mut flow),
            GameSystem::End(system) => system.input(input, &mut flow),
        }
        flow.apply(self);
    }

    pub fn update(&mut self) {
        let mut flow = GameSystemFlow::Default;
        match self {
            GameSystem::Start(system) => system.update(&mut flow),
            GameSystem::Playing(system) => system.update(&mut flow),
            GameSystem::End(system) => system.update(&mut flow),
        }
        flow.apply(self);
    }

    pub fn context(&self) -> GameContext {
        match self {
            GameSystem::Start(system) => GameContext::Start(system.context()),
            GameSystem::Playing(system) => GameContext::Playing(system.context()),
            GameSystem::End(system) => GameContext::End(system.context()),
        }
    }
}

pub enum GameSystemFlow {
    Default,
    To(GameSystem),
}

impl GameSystemFlow {
    pub fn apply(self, system: &mut GameSystem) {
        if let GameSystemFlow::To(new_system) = self {
            *system = new_system;
        }
    }
}
