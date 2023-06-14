use super::*;
use std::{fs, path};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Distance(usize);

const MAX_DISTANCE: usize = 15;

pub fn find_mount_path(
    search_mount_name: &Option<String>,
) -> Result<Vec<String>, EngineIgniteError> {
    let mut current_path = std::env::current_dir().map_err(EngineIgniteError::MountSearchIO)?;
    let mut distance = Distance(1);

    loop {
        let mut report = explore_path(&current_path, distance, search_mount_name)?;
        if !report.is_empty() {
            report.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));

            let lowest_distance = report.first().unwrap().1;

            return Ok(report
                .into_iter()
                .filter_map(|(path, distance)| (lowest_distance == distance).then_some(path))
                .collect());
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
