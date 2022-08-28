use bitflags::bitflags;
use nix::{errno, libc};
use std::collections::HashSet;

mod mappings;

bitflags! {
  struct Namespaces: u32 {
    const NET = libc::CLONE_NEWNET;
    const PID = libc::CLONE_NEWPID;
    const MOUNT = libc::CLONE_NEWNS;
    const USER = libc::CLONE_NEWUSER;
    const IPC = libc::CLONE_NEWIPC;
    const CGROUP = libc::CLONE_NEWCGROUP;
    const UTS = libc::CLONE_NEWUTS;
  }
}

enum UnshareError {
    InvalidArgument,
    NoMemory,
    PermissionDenied,
}

fn unshare(namespaces: Namespaces) -> Result<(), UnshareError> {
    unsafe {
        let exit_code = libc::unshare(namespaces);
        if exit_code == 0 {
            return Ok(());
        }
        let errno = errno::errno();
        match errno {
            libc::EINVAL => return Err(UnshareError::InvalidArgument),
            libc::EPERM => return Err(UnshareError::PermissionDenied),
            libc::ENOMEM => return Err(UnshareError::NoMemory),
            _ => panic!("unexpected errno: {}", errno),
        }
    }
}
