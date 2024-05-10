use std::mem::zeroed;
use std::ptr::null_mut;

#[test]
fn test() {
    unsafe {
        let mut ring = zeroed();

        let res = uringx::io_uring_queue_init(1, &mut ring, 0);
        assert_eq!(res, 0);

        let sqe = uringx::io_uring_get_sqe(&mut ring);
        assert!(!sqe.is_null());

        uringx::io_uring_prep_nop(sqe);
        uringx::io_uring_sqe_set_data64(sqe, 327482);

        let res = uringx::io_uring_submit(&mut ring);
        assert_eq!(res, 1);

        let mut cqe = null_mut();
        let res = uringx::io_uring_wait_cqe(&mut ring, &mut cqe);
        assert_eq!(res, 0);
        assert!(!cqe.is_null());

        assert_eq!((*cqe).res, 0);
        assert_eq!((*cqe).user_data, 327482);

        uringx::io_uring_cqe_seen(&mut ring, cqe);

        uringx::io_uring_queue_exit(&mut ring);
    }
}
