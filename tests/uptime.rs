// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
fn test_uptime() {
    if sysinfo::IS_SUPPORTED {
        let mut s = sysinfo::System::new();
        s.refresh_all();
        assert!(s.uptime() != 0);
    }
}
