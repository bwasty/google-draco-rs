// #![feature(static_nobundle)]

mod bindings;
pub use bindings::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let data = include_bytes!("../draco/testdata/test_nm_quant.0.9.0.drc");
            let mut buf = DecoderBuffer::default();
            buf.Init(data.as_ptr() as *const i8, data.len());
            let mut decoder = Decoder::default();
            // TODO!!!: segfaults -> nullptr arrives for buf..???
            // let ptr = &mut buf as *mut DecoderBuffer;
            decoder.DecodeMeshFromBuffer(&mut buf);
            // Decoder_DecodeMeshFromBuffer(&mut decoder, &mut buf);
        }
    }
}
