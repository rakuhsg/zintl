#[cfg(target_os = "macos")]
unsafe extern "C" {
    pub fn ztloop_init();
    pub fn ztloop_run();
}
