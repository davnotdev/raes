pub enum SceneExit {
    Next(String),
    End,
}

pub trait Scene {
    fn run(&mut self) -> Result<SceneExit, String>;
}
