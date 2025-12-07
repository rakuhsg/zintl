use windows::core::PCWSTR;

pub trait StringExt {
    fn to_utf16(&self) -> Vec<u16>;

    fn to_pcwstr(&self) -> PCWSTR;
}

impl StringExt for String {
    fn to_utf16(&self) -> Vec<u16> {
        self.encode_utf16().chain([0u16]).collect::<Vec<u16>>()
    }

    fn to_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.to_utf16().as_mut_ptr() as *mut _ as _)
    }
}
