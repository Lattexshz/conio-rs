// Link api-ms-win-crt-conio-l1-1-0.dll
#[link(name="api-ms-win-crt-conio-l1-1-0", kind="raw-dylib")]
extern{
    fn _getch() -> i32;
}

// Implementation
pub fn getch() -> i32 {
    unsafe {
        _getch()
    }
}