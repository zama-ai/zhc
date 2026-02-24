pub fn n_bits_to_encode<I, O>(val_to_encode: I) -> O
where
    I: TryInto<f64>,
    usize: TryInto<O>,
{
    let val = match val_to_encode.try_into() {
        Ok(o) => o,
        Err(_) => panic!(),
    };
    let interm: usize = (val + 1.).log2().ceil() as usize;
    match interm.try_into() {
        Ok(o) => o,
        Err(_) => panic!(),
    }
}
