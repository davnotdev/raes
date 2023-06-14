use serde::Deserialize;

mod args;
mod mount;

use args::parse_arguments;
use mount::find_mount_path;

const MOUNT_ROOT_CONFIG_FILE_NAME: &str = "raes.ron";

#[derive(Debug)]
pub enum EngineIgniteError {
    BadArg(String),
    LeftOverArg,
    MountSearchIO(std::io::Error),
    MountSearchRootNotFound,
    MountSearchRootNotFoundNearby,
}

#[derive(Deserialize)]
struct EngineConfig {}

pub struct Engine {}

impl Engine {
    pub fn ignite() -> Result<Self, EngineIgniteError> {
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        let args = parse_arguments(&args)?;

        let res = find_mount_path(&args.search_mount_name).unwrap();
        eprintln!("Ignition! {:?}", res);

        todo!()
    }
}

#[derive(Default)]
struct EngineArgs {
    mount_path: Option<String>,
    search_mount_name: Option<String>,
    scene: Option<String>,
}
