use crate::fse::Watch;
use anyhow::Result;
use duration_string::DurationString;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct State {
  pub bases: Bases,
  pub units: Vec<Units>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Bases {
  pub backs: PathBuf,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Units {
  pub model: Model,
  metal: Metal,
  views: Views,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Model {
  pub ident: String,
  pub title: String,
  pub paths: PathBuf,
  pub backs: PathBuf,
  pub waits: DurationString,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Metal {
  #[serde(skip)]
  watch: Option<Arc<Watch>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Views {
  watch: bool,
  files: Vec<PathBuf>,
}

impl Units {
  pub fn new(model: Model) -> Result<Self> {
    let units = Self {
      model,
      metal: Metal::default(),
      views: Views::default(),
    };
    Ok(units)
  }

  pub fn setup(&mut self) -> Result<()> {
    let watch = Watch::watch(&self.model.paths, self.model.waits.into(), self.hooks())?;
    self.metal.watch = Some(Arc::new(watch));
    Ok(())
  }

  fn catch(f: impl Fn() -> Result<()>) {
    let _ = f().map_err(|e| log::error!("catch error: {:?}", e));
  }

  fn hooks(&self) -> impl Fn() + Send + 'static {
    let model = self.model.clone();
    move || {
      Self::catch(|| {
        log::debug!("watch hooks: {:?}", model);
        std::fs::create_dir_all(&model.backs)?;
        Self::mkzip(&model.paths, &model.backs)?;
        Ok(())
      })
    }
  }

  fn mkzip(paths: &PathBuf, backs: &PathBuf) -> Result<PathBuf> {
    let times = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S");
    let files = backs.join(format!("A-{}.zip", times));
    log::debug!("mkzip: begin {:?} -> {:?}", paths, files);
    zip_extensions::zip_create_from_directory(&files, paths)?;
    log::debug!("mkzip: ended {:?} -> {:?}", paths, files);
    Ok(files)
  }
}

impl Units {
  pub fn fresh(&mut self) -> Result<()> {
    self.views = self.views()?;
    Ok(())
  }

  fn views(&self) -> Result<Views> {
    let watch = self.metal.watch.is_some();
    let files = self.files()?;
    Ok(Views { watch, files })
  }

  fn files(&self) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(&self.model.backs)?;
    let files = std::fs::read_dir(&self.model.backs)?;
    let files = files.filter_map(|x| x.ok());
    let files = files.filter(|x| x.path().extension().map_or(false, |x| x == "zip"));
    let files = files.sorted_by_key(|x| x.metadata().ok()?.modified().ok());
    let files = files.rev().map(|x| x.path());
    Ok(files.collect())
  }
}

impl Drop for Units {
  fn drop(&mut self) {
    log::debug!("watch clean: {:?}", self.model);
  }
}
