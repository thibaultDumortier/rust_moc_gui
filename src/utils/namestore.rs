use std::{
    collections::HashMap,
    sync::{Once, RwLock},
};

/// Function used only once to init the store
static NAME_STORE_INIT: Once = Once::new();
/// The MOC store (a simple hasmap), protected from concurrent access by a RwLock.
static mut NAME_STORE: Option<RwLock<HashMap<usize, (String, usize)>>> = None;
static mut LATEST_IDX: usize = 0;

/// Get (or create and get) the read/write protected MOC store
/// All read/write  operations on the store have to call this method.
pub(crate) fn get_store() -> &'static RwLock<HashMap<usize, (String, usize)>> {
    unsafe {
        // Inspired from the Option get_or_insert_with method, modified to ensure thread safety with
        // https://doc.rust-lang.org/std/sync/struct.Once.html
        // This implements a double-checked lock.
        if NAME_STORE.is_none() {
            NAME_STORE_INIT.call_once(|| {
                NAME_STORE = Some(RwLock::new(HashMap::new()));
            });
        }
        match &NAME_STORE {
            Some(v) => v,
            None => unreachable!(),
        }
    }
}
pub(crate) fn drop(id: usize) -> Result<(), String> {
    let mut store = get_store()
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    (*store).remove(&id);

    Ok(())
}
pub(crate) fn add(name: &str, id: usize) -> Result<(), String> {
    let new_idx: usize = get_latest_idx();
    let idx = list_names()
        .unwrap()
        .iter()
        .filter(|s| s.contains(name))
        .count();

    let mut store = get_store()
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;

    if idx != 0 {
        (*store).insert(id, (format!("{}({})", name, idx), new_idx));
    } else {
        (*store).insert(id, (String::from(name), new_idx));
    }

    Ok(())
}
pub(crate) fn list_names() -> Result<Vec<String>, String> {
    Ok(get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?
        .iter()
        .map(|(_, name)| name.0.clone())
        .collect())
}

/////////////
// GETTERS //

pub(crate) fn get_name(id: usize) -> Result<String, String> {
    let store = get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?;
    let name = store
        .get(&id)
        .ok_or_else(|| format!("MOC '{}' not found", id))?;

    Ok(name.0.to_owned())
}
pub fn get_len() -> Result<usize, String> {
    Ok(get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?
        .len())
}
pub(crate) fn get_last(index: usize) -> Result<(usize, String), String> {
    let len = get_len().unwrap() - (index + 1);
    let binding = get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?;
    let last = binding.get(&len).unwrap();

    Ok((len, last.0.to_owned()))
}
fn get_latest_idx() -> usize {
    unsafe {
        let li = LATEST_IDX;
        LATEST_IDX += 1;
        li
    }
}
