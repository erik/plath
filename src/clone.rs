use libc::{c_int, c_void, pid_t};

/// Flags that clone(2) uses. For more verbose descriptions, see the manpage.
///
/// There are some other flags that can be passed, but they're useless for our
/// purposes, so I won't add them here.
pub mod flags {
    use libc::c_int;

    /// This is how we'll be calling clone for threading purposes.
    pub const COMMON: c_int = VM | FS | FILES | SIGHAND | THREAD | // SETTLS |
                              PARENT_SETTID | CHILD_CLEARTID | SYSVSEM;

    /// Set if VM shared between processes.
    const VM: c_int = 0x00000100;
    /// Set if fs info shared between processes.
    const FS: c_int = 0x00000200;
    /// Set if open files shared between processes.
    const FILES: c_int = 0x00000400;
    /// Set if signal handlers shared.
    const SIGHAND: c_int = 0x00000800;
    /// Set if we want to have the same parent as the cloner.
    const PARENT: c_int = 0x00008000;
    /// Set to add to same thread group.
    const THREAD: c_int = 0x00010000;
    /// Set to shared SVID SEM_UNDO semantics.
    const SYSVSEM: c_int = 0x00040000;
    /// Set TLS info.
    const SETTLS: c_int = 0x00080000;
    /// Store TID in userlevel buffer before MM copy.
    const PARENT_SETTID: c_int = 0x00100000;
    /// Register exit futex and memory location to clear.
    const CHILD_CLEARTID: c_int = 0x00200000;
    /// Create clone detached.
    const DETACHED: c_int = 0x00400000;
    /// Store TID in userlevel buffer in the child.
    const CHILD_SETTID: c_int = 0x01000000;
}

pub type CloneFunc = extern fn (arg: *mut c_void) -> c_int;

extern "C" {
    /// Wraps the clone(2) function (defined in glibc as
    /// /sysdeps/unix/sysv/linux/ARCH/clone.S)
    ///
    /// int clone(int (*fn)(void *), void *child_stack, int flags, void *arg,
    ///           pid_t *ptid, struct user_desc *tls, pid_t *ctid)
    pub fn clone(func: CloneFunc,
                 child_stack: *mut c_void,
                 flag: c_int,
                 arg: *mut c_void,
                 ptid: *mut pid_t,
                 tls: *mut c_void, // TODO: handle this somehow
                 ctid: *mut pid_t) -> c_int;
}
