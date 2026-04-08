use std::sync::mpsc::{Receiver, channel};

use camino::Utf8PathBuf;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify::{Event, EventKind, RecursiveMode, Result, recommended_watcher};
use tracing::warn;

use crate::runner::watcher::{Watcher, WatcherEvent};

pub(crate) struct DefaultWatcher {
    rx: Receiver<Result<Event>>,
    watcher: Box<dyn notify::Watcher>,
}

impl Watcher for DefaultWatcher {
    fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            rx,
            watcher: Box::new(recommended_watcher(tx).expect("watcher created")),
        }
    }

    fn watch(&mut self, paths: impl IntoIterator<Item = Utf8PathBuf>) {
        let mut watched_paths = self.watcher.paths_mut();

        for path in paths {
            if let Err(e) = watched_paths.add(path.as_std_path(), RecursiveMode::Recursive) {
                warn!("Failed to watch path {}: {}", path, e);
            }
        }
        if let Err(e) = watched_paths.commit() {
            warn!("Failed to commit watched paths: {}", e);
        }
    }

    fn poll(&mut self) -> Option<WatcherEvent> {
        let event = self.rx.iter().filter_map(Result::ok).find(|event| {
            // Modifying folder or metadata is ignored as it can unlikely affect the results.
            // Any event types are necessary for some platforms to catch events.
            matches!(
                event.kind,
                EventKind::Create(CreateKind::File | CreateKind::Any)
                    | EventKind::Modify(
                        ModifyKind::Name(_) | ModifyKind::Data(_) | ModifyKind::Any
                    )
                    | EventKind::Remove(RemoveKind::File | RemoveKind::Any)
                    | EventKind::Any
            )
        })?;

        Some(WatcherEvent {
            paths: event
                .paths
                .into_iter()
                .filter_map(|path| Utf8PathBuf::from_path_buf(path).ok())
                .collect(),
        })
    }
}
