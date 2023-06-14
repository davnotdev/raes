use super::*;

pub(super) fn parse_arguments(args: &[String]) -> Result<EngineArgs, EngineIgniteError> {
    enum EngineFlags {
        Scene,
        MountPath,
        SearchMountName,
    }

    let mut output = EngineArgs::default();
    let mut previous_carried_flag = None;
    for arg in args {
        if let Some(ref previous_carried) = previous_carried_flag {
            match previous_carried {
                EngineFlags::Scene => output.scene = Some(arg.clone()),
                EngineFlags::MountPath => output.mount_path = Some(arg.clone()),
                EngineFlags::SearchMountName => output.search_mount_name = Some(arg.clone()),
            }
            previous_carried_flag = None;
        } else {
            match arg.as_str() {
                "--scene" | "-s" => {
                    previous_carried_flag = Some(EngineFlags::Scene);
                }
                "--mount" | "-n" => {
                    previous_carried_flag = Some(EngineFlags::SearchMountName);
                }
                "--mount-path" | "-m" => {
                    previous_carried_flag = Some(EngineFlags::MountPath);
                }
                flag => Err(EngineIgniteError::BadArg(format!(
                    "Unrecognized flag: `{}`.",
                    flag
                )))?,
            }
        }
    }

    if previous_carried_flag.is_some() {
        Err(EngineIgniteError::LeftOverArg)?
    }

    Ok(output)
}
