use libc::{c_int, c_void};

/// Set if VM shared between processes.
pub const VM             : c_int = 0x00000100;
/// Set if fs info shared between processes.
pub const FS             : c_int = 0x00000200;
/// Set if open files shared between processes.
pub const FILES          : c_int = 0x00000400;
/// Set if signal handlers shared.
pub const SIGHAND        : c_int = 0x00000800;
/// Set if tracing continues on the child.
pub const PTRACE         : c_int = 0x00002000;
/// Set if the parent wants the child to wake it up on mm_release.
pub const VFORK          : c_int = 0x00004000;
/// Set if we want to have the same parent as the cloner.
pub const PARENT         : c_int = 0x00008000;
/// Set to add to same thread group.
pub const THREAD         : c_int = 0x00010000;
/// Set to create new namespace.
pub const NEWNS          : c_int = 0x00020000;
/// Set to shared SVID SEM_UNDO semantics.
pub const SYSVSEM        : c_int = 0x00040000;
/// Set TLS info.
pub const SETTLS         : c_int = 0x00080000;
/// Store TID in userlevel buffer before MM copy.
pub const PARENT_SETTID  : c_int = 0x00100000;
/// Register exit futex and memory location to clear.
pub const CHILD_CLEARTID : c_int = 0x00200000;
/// Create clone detached.
pub const DETACHED       : c_int = 0x00400000;
/// Set if the tracing process can't force PTRACE on this clone.
pub const UNTRACED       : c_int = 0x00800000;
/// Store TID in userlevel buffer in the child.
pub const CHILD_SETTID   : c_int = 0x01000000;
/// Start in stopped state.
pub const STOPPED        : c_int = 0x02000000;


extern {
    /// Wraps the clone(2) function (glibc/<arch>/clone.S)
    pub fn clone(func: extern fn (arg: *mut c_void) -> c_int,
                 child_stack: *mut c_void,
                 flag: c_int,
                 arg: *mut c_void) -> c_int;
}
