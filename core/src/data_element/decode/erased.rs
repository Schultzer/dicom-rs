//! This module contains the type-erased version of a decoder.

use std::io::Read;
use error::Result;
use data_element::{DataElementHeader, SequenceItemHeader};
use std::fmt::Debug;

/** Type trait for reading and decoding DICOM data elements.
 * 
 * The specific behaviour of decoding, even when abstracted from the original source,
 * may depend on the given transfer syntax. As each element is retrieved, a temporary
 * cursor to the data is obtained, allowing for an optional reading of the full contents
 * of the element.
 * 
 * This is the type-erased version of `super::Decode`, where the data source type is not
 * known in compile time. Users of this library should not need to rely on this level
 * directly, as the given implementations provide support for converting a generic decoder
 * to a type-erased decoder and vice versa.
 */
pub trait Decode: Debug {
    /** Fetch and decode the next data element header from the given source.
     * This method returns only the header of the element. At the end of this operation, the source
     * will be pointing at the element's value data, which should be read or skipped as necessary.
     */
    fn erased_decode(&self, source: &mut Read) -> Result<DataElementHeader>;

    /** Fetch and decode the next sequence item head from the given source.
     * This method returns only the header of the item. At the end of this operation, the source
     * will be pointing at the beginning of the item's data, which should be traversed if necessary.
     */
    fn erased_decode_item(&self, source: &mut Read) -> Result<SequenceItemHeader>;
}