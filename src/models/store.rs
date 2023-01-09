use std::collections::HashMap;
use std::sync::{Once, RwLock};

use crate::commons::{InternalMoc, Qty};

use super::img::to_img_default;

/// Fonction used only once to init the store
static MOC_STORE_INIT: Once = Once::new();
/// The MOC store (a simple hasmap), protected from concurrent access by a RwLock.
static mut MOC_STORE: Option<RwLock<HashMap<String, InternalMoc>>> = None;

/// Get (or create and get) the read/write protected MOC store
/// All read/write  operations on the store have to call this method.
pub(crate) fn get_store() -> &'static RwLock<HashMap<String, InternalMoc>> {
    unsafe {
        // Inspired from the Option get_or_insert_with method, modified to ensure thread safety with
        // https://doc.rust-lang.org/std/sync/struct.Once.html
        // This implements a double-checked lock.
        if MOC_STORE.is_none() {
            MOC_STORE_INIT.call_once(|| {
                MOC_STORE = Some(RwLock::new(HashMap::new()));
            });
        }
        match &MOC_STORE {
            Some(v) => v,
            None => unreachable!(),
        }
    }
}

pub(crate) fn get_img(name: &str, size: (u16, u16)) -> Result<Vec<u8>, String> {
    let store = get_store();

    let store = store.read().map_err(|_| "Read lock poisoned".to_string())?;
    let moc = store
        .get(name)
        .ok_or_else(|| format!("MOC '{}' not found", name))?;
    match moc {
        InternalMoc::Space(smoc) => Ok(to_img_default(smoc, size, None, None)),
        _ => Err("NOT SUPPOSED TO HAPPEN".to_string()),
    }
}

// UNUSED ATM
pub(crate) fn get_info(name: &str) -> Result<String, String> {
    let store = get_store();

    let store = store.read().map_err(|_| "Read lock poisoned".to_string())?;
    let moc = store
        .get(name)
        .ok_or_else(|| format!("MOC '{}' not found", name))?;

    Ok("hi".to_string())
}

pub(crate) fn get_qty(name: &str) -> Result<Qty, String> {
    let store = get_store();
    // Perform read operations first
    let res_qty = {
        let store = store.read().map_err(|_| "Read lock poisoned".to_string())?;
        let moc = store
            .get(name)
            .ok_or_else(|| format!("MOC '{}' not found", name))?;
        match moc {
            InternalMoc::Space(_) => Qty::Space,
            InternalMoc::Time(_) => Qty::Time,
            InternalMoc::TimeSpace(_) => Qty::Timespace,
        }
    };
    Ok(res_qty)
}

/// Add a new MOC to the store
pub(crate) fn add(name: &str, moc: InternalMoc) -> Result<(), String> {
    let mut store = get_store()
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    (*store).insert(String::from(name), moc);
    Ok(())
}

pub(crate) fn drop(name: &str) -> Result<(), String> {
    let mut store = get_store()
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    (*store).remove(name);
    Ok(())
}

/// Returns the MOCs identifiers (names)
pub(crate) fn list_mocs() -> Result<Vec<String>, String> {
    Ok(get_store()
        .read()
        .map_err(|_| "Read lock poisoned".to_string())?
        .iter()
        .map(|(key, _)| key.clone())
        .collect())
}

pub(crate) fn exec<R, F>(name: &str, op: F) -> Option<R>
where
    R:,
    F: Fn(&InternalMoc) -> R,
{
    get_store().read().unwrap().get(name).map(op)
}

/// Perform an operation on a MOC and store the resulting MOC.
pub(crate) fn op1<F>(name: &str, op: F, res_name: &str) -> Result<(), String>
where
    F: Fn(&InternalMoc) -> Result<InternalMoc, String>,
{
    let store = get_store();
    // Perform read operations first
    let res_moc = {
        let store = store.read().map_err(|_| "Read lock poisoned".to_string())?;
        let moc = store
            .get(name)
            .ok_or_else(|| format!("MOC '{}' not found", name))?;
        op(moc)?
    };
    // Then write operation.
    // Remark: we could have called directly add(res_name, res_moc)
    //         (still carefully releasing the read lock before the call),
    //         but we (so far) preferred to spare one `get_store` call
    let mut store = store
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    (*store).insert(String::from(res_name), res_moc);
    Ok(())
}

/// Perform an operation on a MOC and store the resulting MOC.
pub(crate) fn op1_multi_res<F>(name: &str, op: F, res_name_prefix: &str) -> Result<(), String>
where
    F: Fn(&InternalMoc) -> Result<Vec<InternalMoc>, String>,
{
    let store = get_store();
    // Perform read operations first
    let res_mocs = {
        let store = store.read().map_err(|_| "Read lock poisoned".to_string())?;
        let moc = store
            .get(name)
            .ok_or_else(|| format!("MOC '{}' not found", name))?;
        op(moc)?
    };
    // Then write operation.
    // Remark: we could have called directly add(res_name, res_moc)
    //         (still carefully releasing the read lock before the call),
    //         but we (so far) preferred to spare one `get_store` call
    let mut store = store
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    for (i, res_moc) in res_mocs.into_iter().enumerate() {
        (*store).insert(format!("{}_{}", res_name_prefix, i), res_moc);
    }
    Ok(())
}

/// Perform an operation between 2 MOCs and store the resulting MOC.
pub(crate) fn op2<F>(left_name: &str, right_name: &str, op: F, res_name: &str) -> Result<(), String>
where
    F: Fn(&InternalMoc, &InternalMoc) -> Result<InternalMoc, String>,
{
    let store = get_store();
    // Perform read operations first
    let res_moc = {
        let store = store.read().map_err(|_| "Read lock poisoned".to_string())?;
        let left = store
            .get(left_name)
            .ok_or_else(|| format!("MOC '{}' not found", left_name))?;
        let right = store
            .get(right_name)
            .ok_or_else(|| format!("MOC '{}' not found", right_name))?;
        op(left, right)?
    };
    // Then write operation.
    // Remark: we could have called directly add(res_name, res_moc)
    //         (still carefully releasing the read lock before the call),
    //         but we (so far) preferred to spare one `get_store` call
    let mut store = store
        .write()
        .map_err(|_| "Write lock poisoned".to_string())?;
    (*store).insert(String::from(res_name), res_moc);
    Ok(())
}
