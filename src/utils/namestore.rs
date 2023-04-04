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

////////////////
// OPERATIONS //

//////////////////////////////////////////////////////////////////////////
// #Errors
//      All functions have the same error, if the store is already open for writing
//      then the lock is poisoned and the store can't be written on.
//
//      List names is different, the error comes from the reading lock.
//////////////////////////////////////////////////////////////////////////

// #Definition
//      drop simply drops a name from the namestore.
// #Args
//  *   `id`: a given id of the MOC to drop
pub(crate) fn drop(id: usize) -> Result<(), String> {
    let mut store = get_store()
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    (*store).remove(&id);

    Ok(())
}
// #Definition
//      add simply adds a name from the namestore.
// #Args
//  *   `name`: the name of the newly added MOC
//  *   `id`: a given id of the MOC to add
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
        (*store)
            .entry(id)
            .or_insert((format!("{name}({idx})"), new_idx));
    } else {
        (*store).entry(id).or_insert((name.to_string(), new_idx));
    }

    Ok(())
}
// #Definition
//      list_names simply gives all names currently stored.
pub(crate) fn list_names() -> Result<Vec<String>, String> {
    Ok(get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?
        .iter()
        .map(|(_, name)| name.0.clone())
        .collect())
}
pub(crate) fn list_ids() -> Result<Vec<usize>, String> {
    Ok(get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?
        .iter()
        .map(|(id, _)| *id)
        .collect())
}

pub(crate) fn rename(id: usize, name: &str) -> Result<(), String> {
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
        (*store).insert(id, (format!("{name}({idx})"), new_idx));
    } else {
        (*store).insert(id, (name.to_string(), new_idx));
    }

    Ok(())
}

/////////////
// GETTERS //

// #Definition
//      get_name gets the name of a given MOC based on id
// #Args
//  *   `id`: a given id of the MOC to get
pub(crate) fn get_name(id: usize) -> Result<String, String> {
    let store = get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?;
    let name = store
        .get(&id)
        .ok_or_else(|| format!("MOC '{id}' not found in get_name"))?;

    Ok(name.0.to_owned())
}
// #Definition
//      get_last gets the last name stored in the namestore.
// #Args
//  *   `index`: the specific index in case the function needs to search
//               a MOC before the last one (for example the second to last)
pub(crate) fn get_last() -> Result<(usize, String), String> {
    let binding = get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?;
    let last_key = *binding.keys().last().unwrap();
    let last = binding.get(&last_key).unwrap();
    Ok((last_key, last.0.to_owned()))
}

pub(crate) fn get_before_last() -> Result<(usize, String), String> {
    let binding = get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?;
    let last_key = *binding.keys().last().unwrap();

    let mut beforelast_key: usize = 0;
    for key in binding.keys() {
        if key != &last_key {
            beforelast_key = *key;
        } else {
            break;
        }
    }
    let beforelast = binding.get(&beforelast_key).unwrap();

    Ok((beforelast_key, beforelast.0.to_owned()))
}
// #Definition
//      get_latest_idx gets a new index to add to the newly added MOC.
//      It's used to sort MOCs in the list UI in loading order and not in hashmap order.
fn get_latest_idx() -> usize {
    unsafe {
        let li = LATEST_IDX;
        LATEST_IDX += 1;
        li
    }
}
