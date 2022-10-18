use moc::{
    deser::ascii::{from_ascii_ivoa, moc2d_from_ascii_ivoa},
    moc::{CellOrCellRangeMOCIntoIterator, CellOrCellRangeMOCIterator, RangeMOCIterator},
    moc2d::{CellOrCellRangeMOC2IntoIterator, RangeMOC2IntoIterator, RangeMOC2Iterator},
    qty::{Hpx, Time},
};
use wasm_bindgen::JsValue;

use crate::commons::InternalMoc;

pub(crate) fn smoc_from_ascii(data: &str) -> Result<InternalMoc, JsValue> {
    let cellcellranges =
        from_ascii_ivoa::<u64, Hpx<u64>>(data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let moc = cellcellranges
        .into_cellcellrange_moc_iter()
        .ranges()
        .into_range_moc();
    Ok(InternalMoc::Space(moc))
}

pub(crate) fn tmoc_from_ascii(data: &str) -> Result<InternalMoc, JsValue> {
    let cellcellranges =
        from_ascii_ivoa::<u64, Time<u64>>(data).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let moc = cellcellranges
        .into_cellcellrange_moc_iter()
        .ranges()
        .into_range_moc();
    Ok(InternalMoc::Time(moc))
}

pub(crate) fn stmoc_from_ascii(data: &str) -> Result<InternalMoc, JsValue> {
    let cellrange2 = moc2d_from_ascii_ivoa::<u64, Time<u64>, u64, Hpx<u64>>(data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let moc2 = cellrange2
        .into_cellcellrange_moc2_iter()
        .into_range_moc2_iter()
        .into_range_moc2();
    Ok(InternalMoc::TimeSpace(moc2))
}
