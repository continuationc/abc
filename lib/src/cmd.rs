use crate::set::Model;
use crate::set::State;
use crate::set::Units;
use anyhow::anyhow;
use anyhow::Result;
use std::sync::Mutex;
use tauri::ipc::Invoke;
use tauri::AppHandle;
use tauri::Emitter;

type AppState<'a, T> = tauri::State<'a, Mutex<T>>;

pub fn new() -> impl Fn(Invoke) -> bool {
  tauri::generate_handler![greet, fresh, start, close]
}

#[tauri::command]
pub fn greet(input: &str) -> String {
  format!("Hello {}", input)
}

#[tauri::command]
pub fn fresh(apple: AppHandle, state: AppState<State>) -> Result<(), ()> {
  catch(|| {
    let mut state = state.lock().map_err(|e| anyhow!("mutex error: {:?}", e))?;
    state.units.iter_mut().try_for_each(|x| x.fresh())?;
    apple.emit("state", state.clone())?;
    log::debug!("state: {:?}", state);
    Ok(())
  })
}

#[tauri::command]
pub fn start(apple: AppHandle, state: AppState<State>, model: Model) -> Result<(), ()> {
  catch(|| {
    log::debug!("start: {:?}", model);
    let mut units = Units::new(model.clone())?;
    units.setup()?;
    units.fresh()?;
    let mut state = state.lock().map_err(|e| anyhow!("mutex error: {:?}", e))?;
    state.units.push(units);
    apple.emit("state", state.clone())?;
    log::debug!("state: {:?}", state);
    Ok(())
  })
}

#[tauri::command]
pub fn close(apple: AppHandle, state: AppState<State>, model: Model) -> Result<(), ()> {
  catch(|| {
    log::debug!("close: {:?}", model);
    let mut state = state.lock().map_err(|e| anyhow!("mutex error: {:?}", e))?;
    state.units.retain(|x| x.model.ident != model.ident);
    apple.emit("state", state.clone())?;
    log::debug!("state: {:?}", state);
    Ok(())
  })
}

fn catch(f: impl Fn() -> Result<()>) -> Result<(), ()> {
  f().map_err(|e| log::error!("catch error: {:?}", e))
}
