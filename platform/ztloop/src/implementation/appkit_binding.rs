#[cfg(target_os = "macos")]
unsafe extern "C" {
    pub fn ztloopa_init();
    pub fn ztloopa_run();
    pub fn ztloopa_destroy();
}
