use super::*;
use std::{env, fs, path};

pub fn fs_platform_ignite() -> Result<Engine, EngineIgniteError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = parse_arguments(&args)?;

    let mount_path = if let Some(mount_path) = args.mount_path {
        mount_path
    } else {
        find_mount_path(&args.search_mount_name)?
    };

    env::set_current_dir(&mount_path).unwrap();

    let mut config_path = path::PathBuf::from(mount_path);
    config_path.push(MOUNT_ROOT_CONFIG_FILE_NAME);

    let config_str = fs::read_to_string(config_path).map_err(EngineIgniteError::MountSearchIO)?;

    let config: EngineConfig =
        ron::from_str(&config_str).map_err(|e| EngineIgniteError::ParseConfig(format!("{}", e)))?;

    Ok(Engine { config })
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Distance(usize);

const MAX_DISTANCE: usize = 15;

fn find_mount_path(search_mount_name: &Option<String>) -> Result<String, EngineIgniteError> {
    let mut current_path = env::current_dir().map_err(EngineIgniteError::MountSearchIO)?;
    let mut distance = Distance(1);

    loop {
        let report = explore_path(&current_path, distance, search_mount_name)?;
        if !report.is_empty() {
            if report.len() != 1 {
                let err = report.iter().cloned().map(|(i, _)| i).collect();
                Err(EngineIgniteError::MountAmbiguousRoots(err))?
            }
            return Ok(report.first().unwrap().0.clone());
        }

        distance.0 += 1;

        if !current_path.pop() {
            break;
        }
    }

    Err(EngineIgniteError::MountSearchRootNotFound)
}

fn explore_path(
    path: &path::Path,
    Distance(distance): Distance,
    search_mount_name: &Option<String>,
) -> Result<Vec<(String, Distance)>, EngineIgniteError> {
    if distance > MAX_DISTANCE {
        Err(EngineIgniteError::MountSearchRootNotFoundNearby)?
    }

    let mut out = vec![];
    let dirs = fs::read_dir(path).map_err(EngineIgniteError::MountSearchIO)?;

    for item in dirs {
        let item = item.map_err(EngineIgniteError::MountSearchIO)?;
        let item_path = item.path();
        let item_name = item.file_name();
        let item_type = item.file_type().map_err(EngineIgniteError::MountSearchIO)?;

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
