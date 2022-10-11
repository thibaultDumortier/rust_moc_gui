use std::{error::Error, io::Cursor};

use moc::{
    deser::fits::{MocQtyType, MocType},
    elemset::range::MocRanges,
    idx::Idx,
    moc::{
        range::{op::convert::convert_to_u64, RangeMOC},
        CellMOCIntoIterator, CellMOCIterator, RangeMOCIntoIterator, RangeMOCIterator,
    },
    qty::Hpx,
};
use unreachable::UncheckedResultExt;

pub(crate) type SMOC = RangeMOC<u64, Hpx<u64>>;

#[derive(Clone)]
pub(crate) enum InternalMoc {
    Space(SMOC),
}
impl Default for InternalMoc {
    fn default() -> Self {
        InternalMoc::Space(SMOC::new(0, MocRanges::default()))
    }
}
impl InternalMoc {
    pub(crate) fn to_fits(&self) -> Box<[u8]> {
        let mut buf: Vec<u8> = Default::default();
        // Uses unsafe [unchecked_unwrap_ok](https://docs.rs/unreachable/1.0.0/unreachable/trait.UncheckedResultExt.html)
        // for wasm size optimisation.
        // We do it because no I/O error can occurs since we are writing in memory.
        unsafe {
            match self {
                InternalMoc::Space(moc) => moc
                    .into_range_moc_iter()
                    .to_fits_ivoa(None, None, &mut buf)
                    .unchecked_unwrap_ok(),
            }
        }
        buf.into_boxed_slice()
    }
}

pub(crate) fn from_fits<T: Idx>(moc: MocQtyType<T, Cursor<&[u8]>>) -> Result<InternalMoc, Box<dyn Error>> {
    match moc {
        MocQtyType::Hpx(moc) => from_fits_hpx(moc),
        MocQtyType::Time(_) => todo!(),
        MocQtyType::TimeHpx(_) => todo!(),
    }
}

fn from_fits_hpx<T: Idx>(
    moc: MocType<T, Hpx<T>, Cursor<&[u8]>>,
) -> Result<InternalMoc, Box<dyn Error>> {
    let moc: SMOC = match moc {
        MocType::Ranges(moc) => convert_to_u64::<T, Hpx<T>, _, Hpx<u64>>(moc).into_range_moc(),
        MocType::Cells(moc) => {
            convert_to_u64::<T, Hpx<T>, _, Hpx<u64>>(moc.into_cell_moc_iter().ranges())
                .into_range_moc()
        }
    };
    Ok(InternalMoc::Space(moc))
}
