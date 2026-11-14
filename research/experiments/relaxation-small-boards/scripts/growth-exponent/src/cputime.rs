//! Thread CPU time. The Mac carries a load of 20 to 50 from other jobs, so wall time per
//! run is mostly queueing; CPU time of the solving thread is the stable measure. The only unsafe
//! code in the crate, isolated here: one libc call writing into a local struct.

pub fn thread_cpu() -> f64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: clock_gettime writes one timespec into a valid local; CLOCK_THREAD_CPUTIME_ID is
    // supported on macOS 10.12 and later.
    let rc = unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) };
    assert_eq!(rc, 0);
    ts.tv_sec as f64 + ts.tv_nsec as f64 * 1e-9
}
