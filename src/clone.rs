///! Set if VM shared between processes.
pub const VM             : usize = 0x00000100;
///! Set if fs info shared between processes.
pub const FS             : usize = 0x00000200;
///! Set if open files shared between processes.
pub const FILES          : usize = 0x00000400;
///! Set if signal handlers shared.
pub const SIGHAND        : usize = 0x00000800;
///! Set if tracing continues on the child.
pub const PTRACE         : usize = 0x00002000;
///! Set if the parent wants the child to wake it up on mm_release.
pub const VFORK          : usize = 0x00004000;
///! Set if we want to have the same parent as the cloner.
pub const PARENT         : usize = 0x00008000;
///! Set to add to same thread group.
pub const THREAD         : usize = 0x00010000;
///! Set to create new namespace.
pub const NEWNS          : usize = 0x00020000;
///! Set to shared SVID SEM_UNDO semantics.
pub const SYSVSEM        : usize = 0x00040000;
///! Set TLS info.
pub const SETTLS         : usize = 0x00080000;
///! Store TID in userlevel buffer before MM copy.
pub const PARENT_SETTID  : usize = 0x00100000;
///! Register exit futex and memory location to clear.
pub const CHILD_CLEARTID : usize = 0x00200000;
///! Create clone detached.
pub const DETACHED       : usize = 0x00400000;
///! Set if the tracing process can't force PTRACE on this clone.
pub const UNTRACED       : usize = 0x00800000;
///! Store TID in userlevel buffer in the child.
pub const CHILD_SETTID   : usize = 0x01000000;
///! Start in stopped state.
pub const STOPPED        : usize = 0x02000000;

pub unsafe fn clone(thd: extern fn() -> (), stack: *mut u8, flags: usize) -> u64 {
    //let retval = syscall!(CLONE, thd, flags);

    0
}
