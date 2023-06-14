use serde::Deserialize;

mod args;

/// Implementations for platforms that have a filesystem.
mod fs_platform;

use args::parse_arguments;
use fs_platform::fs_platform_ignite;

const MOUNT_ROOT_CONFIG_FILE_NAME: &str = "raes.ron";

#[derive(Debug)]
pub enum EngineIgniteError {
    BadArg(String),
    LeftOverArg,
    MountSearchIO(std::io::Error),
    MountSearchRootNotFound,
    MountSearchRootNotFoundNearby,
    MountAmbiguousRoots(Vec<String>),

    ParseConfig(String),
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

#[derive(Debug)]
pub struct Engine {
    config: EngineConfig,
}

impl Engine {
    pub fn ignite() -> Result<Self, EngineIgniteError> {
        fs_platform_ignite()
    }
}
