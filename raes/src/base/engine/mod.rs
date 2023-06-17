use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

mod args;
mod scene;

/// Implementations for platforms that have a filesystem.
mod fs_platform;

use args::parse_arguments;
use fs_platform::{
    fs_platform_get_args, fs_platform_get_config_str, fs_platform_load_scene_str,
    fs_platform_write_scene,
};

pub use scene::{IceBox, Preservable, Scene, SceneExit};

const MOUNT_ROOT_CONFIG_FILE_NAME: &str = "raes.ron";

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("At ignite: Got bad argument `{0}`.")]
    IgniteBadArg(String),
    #[error("At ignite: Get ending flag without value.")]
    IgniteLeftOverArg,
    #[error("While mounting: Got an IO error.")]
    MountSearchIO(std::io::Error),
    #[error("While mounting: Project root not found.")]
    MountSearchRootNotFound,
    #[error("While mounting: Project root not found (nearby).")]
    MountSearchRootNotFoundNearby,
    #[error("While mounting: Found multiple project roots: `{0:?}`.")]
    MountAmbiguousRoots(Vec<String>),
    #[error("While loading scene: Got an IO error.")]
    SceneLoadIO(std::io::Error),
    #[error("While writing scene: Got an IO error.")]
    SceneWriteIO(std::io::Error),
    #[error("SceneNotFound (This error should never be recieved.)")]
    SceneNotFound,
    #[error("Scene parse error: `{0}`.")]
    SceneParse(String),
    #[error("Engine config parse error: `{0}`.")]
    ParseConfig(String),
    #[error("Loaded scene has not yet been added.")]
    SceneNotAdded(String),
}

#[derive(Default)]
struct EngineArgs {
    mount_path: Option<String>,
    search_mount_name: Option<String>,
    scene: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EngineConfig {
    load_scene: String,
}

type SceneLoader = fn(&str) -> Result<Box<dyn Scene>, EngineError>;
type SceneDefaultWrite = fn(&str) -> Result<Box<dyn Scene>, EngineError>;

struct SceneData {
    loader: SceneLoader,
    default_write: SceneDefaultWrite,
}

pub struct Engine {
    config: EngineConfig,
    scenes: HashMap<String, SceneData>,
}

impl Engine {
    pub fn ignite() -> anyhow::Result<Self> {
        let args = fs_platform_get_args()?;
        let config_str = fs_platform_get_config_str(&args)?;

        let mut config: EngineConfig =
            ron::from_str(&config_str).map_err(|e| EngineError::ParseConfig(format!("{}", e)))?;

        if let Some(scene) = args.scene {
            config.load_scene = scene;
        }

        let scenes = HashMap::new();

        Ok(Engine { config, scenes })
    }

    pub fn add_scene<S: Scene + Serialize + DeserializeOwned + Default + 'static>(
        &mut self,
        scene_names: &[&str],
    ) -> &mut Self {
        for &scene_name in scene_names {
            self.scenes.insert(
                String::from(scene_name),
                SceneData {
                    loader: |s| {
                        let s: S = ron::from_str(s)
                            .map_err(|e| EngineError::SceneParse(format!("{}", e)))?;
                        Ok(Box::new(s))
                    },
                    default_write: |scene_location| {
                        let s_default = S::default();
                        let s = ron_to_string(&s_default)
                            .map_err(|e| EngineError::SceneParse(e.to_string()))?;
                        fs_platform_write_scene(scene_location, &s)?;
                        Ok(Box::new(s_default))
                    },
                },
            );
        }
        self
    }

    pub fn get_first_scene(&self) -> String {
        self.config.load_scene.clone()
    }

    pub fn run_scene(
        &mut self,
        scene: &str,
        icebox: IceBox,
    ) -> anyhow::Result<Option<(String, IceBox)>> {
        let scene_data = self
            .scenes
            .get(scene)
            .ok_or(EngineError::SceneNotAdded(String::from(scene)))?;

        let mut scene = match fs_platform_load_scene_str(scene) {
            Ok(scene) => (scene_data.loader)(&scene)?,
            Err(e) => {
                if let EngineError::SceneNotFound = e {
                    (scene_data.default_write)(scene)?
                } else {
                    Err(e)?
                }
            }
        };
        let res = match scene.run(icebox)? {
            SceneExit::End => None,
            SceneExit::Next(next, icebox) => Some((next, icebox)),
        };

        Ok(res)
    }
}

fn ron_to_string<S: Serialize + DeserializeOwned>(s: &S) -> Result<String, ron::Error> {
    //  These settings should help fight merge conflicts.
    let config = ron::ser::PrettyConfig::new()
        .struct_names(true)
        .separate_tuple_members(true)
        .compact_arrays(false);
    ron::ser::to_string_pretty(s, config)
}
