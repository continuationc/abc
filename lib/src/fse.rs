use anyhow::Result;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::DebounceEventHandler;
use notify_debouncer_full::Debouncer;
use notify_debouncer_full::FileIdMap;
use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub struct Watch {
  watch: Debouncer<RecommendedWatcher, FileIdMap>,
}

impl Watch {
  pub fn watch(paths: &Path, waits: Duration, hooks: impl Fn() + Send + 'static) -> Result<Watch> {
    let mut watch = new_debouncer(waits, None, Watch::hooks(hooks))?;
    watch.watch(paths, RecursiveMode::Recursive)?;
    log::debug!("watch start: {:?}", watch);
    Ok(Watch { watch })
  }

  fn hooks(hooks: impl Fn() + Send + 'static) -> impl DebounceEventHandler {
    move |r| match r {
      Err(e) => {
        log::error!("watch error: {:?}", e);
      }
      Ok(e) => {
        log::debug!("watch event: {:?}", e);
        hooks();
      }
    }
  }
}

impl Drop for Watch {
  fn drop(&mut self) {
    log::debug!("watch close: {:?}", self.watch);
  }
}
