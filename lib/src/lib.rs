use crate::set::State;
use crate::set::Units;
use anyhow::anyhow;
use std::error::Error;
use std::sync::Mutex;
use tauri::App;
use tauri::Manager;

mod cmd;
mod fse;
mod set;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(set)
    .plugin(log())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
    .manage(Mutex::new(State::default()))
    .invoke_handler(cmd::new())
    .run(tauri::generate_context!())
    .expect("tauri error");
}

fn log() -> impl tauri::plugin::Plugin<tauri::Wry> {
  tauri_plugin_log::Builder::new()
    .level(log::LevelFilter::Debug)
    .level_for("app", log::LevelFilter::Trace)
    .level_for("app_lib", log::LevelFilter::Trace)
    .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
    .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
    .build()
}

fn set(app: &mut App) -> Result<(), Box<dyn Error>> {
  let state = app.state::<Mutex<State>>();
  let mut state = state.lock().map_err(|e| anyhow!("mutex error: {:?}", e))?;
  state.bases.backs = "/Users/user/Documents".parse()?;
  let mut units = Units::default();
  units.model.ident = "test".to_string();
  units.model.title = "test".to_string();
  units.model.paths = "/Users/user/Documents/artificielle/abc/public".parse()?;
  units.model.backs = "/Users/user/Documents/artificielle/abc/logs".parse()?;
  units.model.waits = "2s".parse()?;
  units.setup()?;
  units.fresh()?;
  state.units.push(units);
  log::debug!("state: {:?}", state);
  Ok(())
}
