include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        unsafe {
            let teakra = Teakra_Create();
            Teakra_Reset(teakra);
        }
    }
}
