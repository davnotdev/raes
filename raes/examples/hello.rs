use raes::{asset::*, base::*, surface::*};
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::ignite().unwrap();

    engine.add_scene::<HelloScene>(&["hello.ron"]);

    let mut current_scene = engine.get_first_scene();
    let mut current_icebox = init_icebox();
    while let Some((next_scene, next_icebox)) = engine.run_scene(&current_scene, current_icebox)? {
        current_scene = next_scene;
        current_icebox = next_icebox;
    }

    Ok(())
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
    #[serde(skip)]
    asset_edge: Manual<RwLock<AssetLoaderEdgeData>>,
}

impl Scene for HelloScene {
    fn run(&mut self, mut icebox: IceBox) -> anyhow::Result<SceneExit> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.surface.init(icebox.take().unwrap());
            self.surface_edge
                .init(CopySwap::new(SurfaceEdgeData::new()));
            self.asset_edge
                .init(RwLock::new(AssetLoaderEdgeData::new()));

            let mut text = self.asset_edge.get_mut().load("./text.txt").await?;

            while !self.surface.window_closed() {
                let text = text.get_latest().await;
                eprintln!("Text: {:?}", String::from_utf8(text.to_vec()).unwrap());
            }

            Ok(SceneExit::End)
        })
    }
}
