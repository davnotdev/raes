use crate::base::*;
use std::collections::HashMap;
use tokio::sync::watch;

mod fs_platform;

use fs_platform::{fs_platform_load, fs_platform_watch};

pub struct LoadedData(watch::Receiver<Arc<[u8]>>);

impl LoadedData {
    pub async fn get_latest(&mut self) -> Arc<[u8]> {
        self.0.changed().await.unwrap();
        self.0.borrow_and_update().clone()
    }
}

struct Asset {
    recv: watch::Receiver<Arc<[u8]>>,
}

struct AssetWatcher {
    senders: HashMap<String, Vec<watch::Sender<Arc<[u8]>>>>,
}

pub struct AssetLoaderEdgeData {
    datas: HashMap<String, Asset>,
    watcher_data: Arc<Mutex<AssetWatcher>>,
}

impl AssetLoaderEdgeData {
    pub fn new() -> Self {
        let watcher_data = Arc::new(Mutex::new(AssetWatcher {
            senders: HashMap::new(),
        }));
        let thread_watcher_data = Arc::clone(&watcher_data);
        tokio::task::spawn(async {
            let watcher_data = thread_watcher_data;
            let _ = fs_platform_watch(watcher_data);
        });

        Self {
            datas: HashMap::new(),
            watcher_data,
        }
    }

    pub async fn load(&mut self, path: &str) -> anyhow::Result<LoadedData> {
        //  TODO: Find a cross platform method of resolving file paths for better caching.
        let recv = if let Some(asset) = self.datas.get(path) {
            asset.recv.clone()
        } else {
            let data_bytes = fs_platform_load(path).await?;
            let data: Arc<[u8]> = Arc::from(data_bytes.as_slice());

            let (send, recv) = watch::channel(data.clone());
            let mut watcher_data = self.watcher_data.lock();
            //  First send to signal changed initially.
            send.send(data)?;
            watcher_data
                .senders
                .entry(String::from(path))
                .or_insert(vec![])
                .push(send);
            let ret_recv = recv.clone();
            let asset = Asset { recv };
            self.datas.insert(String::from(path), asset);
            ret_recv
        };
        Ok(LoadedData(recv))
    }
}
