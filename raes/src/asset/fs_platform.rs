use super::*;
use notify::{event::ModifyKind, Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::Path};
use tokio::fs as async_fs;

pub(super) async fn fs_platform_load(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(async_fs::read(path).await?)
}

pub(super) fn fs_platform_watch(watcher_data: Arc<Mutex<AssetWatcher>>) -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    while let Ok(Ok(event)) = rx.recv() {
        if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
            let watcher_data = watcher_data.lock();
            event
                .paths
                .iter()
                .try_for_each(|change_path| -> anyhow::Result<()> {
                    let change_path = fs::canonicalize(change_path)?;
                    watcher_data.senders.iter().try_for_each(
                        |(watch_path, sends)| -> anyhow::Result<()> {
                            let watch_path = fs::canonicalize(watch_path)?;
                            if watch_path == change_path {
                                let new_data = fs::read(watch_path)?;
                                sends.iter().try_for_each(|send| -> anyhow::Result<()> {
                                    send.send(Arc::from(new_data.as_slice()))?;
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })?;
        }
    }

    Ok(())
}
