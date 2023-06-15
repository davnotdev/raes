use raes::{base::*, surface::*};
use serde::{Deserialize, Serialize};

fn main() {
    let mut engine = Engine::ignite().unwrap();

    engine.add_scene::<HelloScene>(&["hello.ron"]);

    let mut current_scene = engine.get_first_scene();
    let mut current_icebox = init_icebox();
    while let Some((next_scene, next_icebox)) =
        engine.run_scene(&current_scene, current_icebox).unwrap()
    {
        current_scene = next_scene;
        current_icebox = next_icebox;
    }
}

fn init_icebox() -> IceBox {
    let mut icebox = IceBox::default();
    icebox.put(Box::new(SurfaceCont::new()));
    icebox
}

#[derive(Default, Serialize, Deserialize)]
pub struct HelloScene {
    #[serde(skip)]
    surface: Manual<Box<SurfaceCont>>,
    #[serde(skip)]
    surface_edge: Manual<CopySwap<SurfaceEdgeData>>,
}

impl Scene for HelloScene {
    fn run(&mut self, mut icebox: IceBox) -> Result<SceneExit, String> {
        self.surface.init(icebox.take().unwrap());
        self.surface_edge
            .init(CopySwap::new(SurfaceEdgeData::new()));

        while !self.surface.window_closed() {}

        Ok(SceneExit::End)
    }
}
