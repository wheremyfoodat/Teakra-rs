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

    #[test]
    fn test_disasm() {
        use std::ffi::CStr;
        use std::os::raw::c_char;

        // Create a buffer for the disassembly text
        let mut buffer = vec![0 as c_char; 256];

        // Opcode and expansion words of our instruction
        let opcode: u16 = 0x1234;
        let expansion: u16 = 0x0000;

        let written =
            unsafe { Teakra_Disasm_Do(buffer.as_mut_ptr(), buffer.len(), opcode, expansion) };

        // Interpret as C string
        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        let rust_str = cstr.to_str().expect("Invalid C string");
        println!("Wrote {} bytes. Instruction: {}", written, rust_str);
    }
}
