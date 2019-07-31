#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            // let mut buf = DecoderBuffer::new(); // TODO!: get actual data...
            let mut decoder = Decoder { options_: DracoOptions::default() };
            // decoder.DecodeMeshFromBuffer(&mut buf);
        }
    }
}
