use raes_base::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct HelloScene {
    number: f32,
}

impl Scene for HelloScene {
    fn run(&mut self) -> Result<SceneExit, String> {
        eprintln!("Beep Boop, Running scene with number: {:?}.", self.number);

        Ok(SceneExit::End)
    }
}

fn main() {
    let mut engine = Engine::ignite().unwrap();
    engine.add_scene::<HelloScene>(&["hello.ron"]);

    let mut current_scene = engine.get_first_scene();

    while let Some(next_scene) = engine.run_scene(&current_scene).unwrap() {
        current_scene = next_scene;
    }
}
