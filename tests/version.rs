#[test]
fn test() {
    unsafe {
        assert_eq!(uringx::io_uring_major_version(), 2);
        assert!(uringx::io_uring_minor_version() >= 4);
    }
}
