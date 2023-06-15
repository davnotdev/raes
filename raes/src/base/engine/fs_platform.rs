use super::*;
use std::{env, fs, io, path};

pub(super) fn fs_platform_get_args() -> Result<EngineArgs, EngineError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    parse_arguments(&args)
}

pub(super) fn fs_platform_get_config_str(args: &EngineArgs) -> Result<String, EngineError> {
    let mount_path = if let Some(mount_path) = &args.mount_path {
        mount_path.to_owned()
    } else {
        find_mount_path(&args.search_mount_name)?
    };

    env::set_current_dir(&mount_path).unwrap();

    let mut config_path = path::PathBuf::from(mount_path);
    config_path.push(MOUNT_ROOT_CONFIG_FILE_NAME);

    fs::read_to_string(config_path).map_err(EngineError::MountSearchIO)
}

pub fn fs_platform_load_scene_str(scene: &str) -> Result<String, EngineError> {
    fs::read_to_string(scene).map_err(|e| {
        if let io::ErrorKind::NotFound = e.kind() {
            EngineError::SceneNotFound
        } else {
            EngineError::SceneLoadIO(e)
        }
    })
}

pub fn fs_platform_write_scene(scene_location: &str, scene: &str) -> Result<(), EngineError> {
    fs::write(scene_location, scene).map_err(EngineError::SceneWriteIO)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Distance(usize);

const MAX_DISTANCE: usize = 4;

fn find_mount_path(search_mount_name: &Option<String>) -> Result<String, EngineError> {
    let mut current_path = env::current_dir().map_err(EngineError::MountSearchIO)?;
    let mut distance = Distance(1);

    loop {
        let report = explore_path(&current_path, distance, search_mount_name)?;
        if !report.is_empty() {
            if report.len() != 1 {
                let err = report.iter().cloned().map(|(i, _)| i).collect();
                Err(EngineError::MountAmbiguousRoots(err))?
            }
            return Ok(report.first().unwrap().0.clone());
        }

        distance.0 += 1;

        if distance.0 > MAX_DISTANCE {
            Err(EngineError::MountSearchRootNotFoundNearby)?
        }

        if !current_path.pop() {
            break;
        }
    }

    Err(EngineError::MountSearchRootNotFound)
}

fn explore_path(
    path: &path::Path,
    Distance(distance): Distance,
    search_mount_name: &Option<String>,
) -> Result<Vec<(String, Distance)>, EngineError> {
    let mut out = vec![];
    let dirs = fs::read_dir(path).map_err(EngineError::MountSearchIO)?;

    for item in dirs {
        let item = item.map_err(EngineError::MountSearchIO)?;
        let item_path = item.path();
        let item_name = item.file_name();
        let item_type = item.file_type().map_err(EngineError::MountSearchIO)?;

        if item_type.is_dir() && path != item_path {
            out.append(&mut explore_path(
                &item_path,
                Distance(distance + 1),
                search_mount_name,
            )?);
        } else if item_type.is_file() && item_name == MOUNT_ROOT_CONFIG_FILE_NAME {
            if let Some(search_mount_name) = search_mount_name {
                if search_mount_name == path.file_name().unwrap().to_str().unwrap() {
                    return Ok(vec![(path.to_str().unwrap().to_owned(), Distance(0))]);
                }
            } else {
                out.push((path.to_str().unwrap().to_owned(), Distance(distance)));
            }
        }
    }

    Ok(out)
}
