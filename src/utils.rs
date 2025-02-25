use std::arch::asm;


#[inline]
pub fn mi_align_up(sz: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two(), "alignment must be a power of two");
    (sz + alignment - 1) & !(alignment - 1)
}

pub fn mi_align_down(sz: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two(), "alignment must be a power of two");
    sz & !(alignment - 1)
}

#[inline]
pub fn mi_thread_id() -> usize {
    let tid: usize;
    unsafe {
        asm!(
            // "mov {}, fs:0",
            "MRS {}, TPIDR_EL0",
            out(reg) tid, 
            options(nostack, nomem)
        );
    }
    tid
}