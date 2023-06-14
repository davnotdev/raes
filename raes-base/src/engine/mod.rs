use super::{Scene, SceneExit};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

mod args;

/// Implementations for platforms that have a filesystem.
mod fs_platform;

use args::parse_arguments;
use fs_platform::{fs_platform_get_args, fs_platform_get_config_str, fs_platform_load_scene_str};

const MOUNT_ROOT_CONFIG_FILE_NAME: &str = "raes.ron";

#[derive(Debug)]
pub enum EngineError {
    IgniteBadArg(String),
    IgniteLeftOverArg,
    MountSearchIO(std::io::Error),
    MountSearchRootNotFound,
    MountSearchRootNotFoundNearby,
    MountAmbiguousRoots(Vec<String>),
    SceneLoadIO(std::io::Error),
    SceneParse(String),
    ParseConfig(String),
    SceneNotAdded(String),
    SceneError(String),
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

pub struct Engine {
    config: EngineConfig,
    scenes: HashMap<String, SceneLoader>,
}

impl Engine {
    pub fn ignite() -> Result<Self, EngineError> {
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

    pub fn add_scene<S: Scene + DeserializeOwned + 'static>(
        &mut self,
        scene_names: &[&str],
    ) -> &mut Self {
        for &scene_name in scene_names {
            self.scenes.insert(String::from(scene_name), |s| {
                let s: S =
                    ron::from_str(s).map_err(|e| EngineError::SceneParse(format!("{}", e)))?;
                Ok(Box::new(s))
            });
        }
        self
    }

    pub fn get_first_scene(&self) -> String {
        self.config.load_scene.clone()
    }

    pub fn run_scene(&mut self, scene: &str) -> Result<Option<String>, EngineError> {
        let scene_loader = self
            .scenes
            .get(scene)
            .ok_or(EngineError::SceneNotAdded(String::from(scene)))?;

        let s = fs_platform_load_scene_str(scene)?;

        let mut scene = scene_loader(&s)?;
        let res = match scene.run().map_err(EngineError::SceneError)? {
            SceneExit::End => None,
            SceneExit::Next(next) => Some(next),
        };

        Ok(res)
    }
}
