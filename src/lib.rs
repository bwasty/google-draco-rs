mod bindings;
pub use bindings::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // unsafe {
            // let mut buf = DecoderBuffer::new(); // TODO!: get actual data...
            let _decoder = Decoder { options_: DracoOptions::default() };
            // decoder.DecodeMeshFromBuffer(&mut buf);
        // }
    }
}
