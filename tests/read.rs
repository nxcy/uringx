//!READ TEST
use std::fs::File;
use std::mem::zeroed;
use std::os::fd::AsRawFd;
use std::ptr::null_mut;

#[test]
fn test() {
    let file = File::open("tests/read.rs").unwrap();
    const LEN: usize = 12;
    unsafe {
        let mut buf = [0u8; LEN];
        let mut ring = zeroed();

        uringx::io_uring_queue_init(1, &mut ring, 0);

        let sqe = uringx::io_uring_get_sqe(&mut ring);
        uringx::io_uring_prep_read(
            sqe,
            file.as_raw_fd(),
            buf.as_mut_ptr() as *mut _,
            LEN as _,
            0,
        );
        uringx::io_uring_submit(&mut ring);

        let mut cqe = null_mut();
        uringx::io_uring_wait_cqe(&mut ring, &mut cqe);

        assert_eq!((*cqe).res, LEN as _);
        assert_eq!(&buf, b"//!READ TEST");
    }
}
