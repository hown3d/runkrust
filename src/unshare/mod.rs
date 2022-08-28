use bitflags::bitflags;
use nix::{errno, libc};

mod mappings;

bitflags! {
  struct Namespaces: u32 {
    const NET = libc::CLONE_NEWNET as u32;
    const PID = libc::CLONE_NEWPID as u32;
    const MOUNT = libc::CLONE_NEWNS as u32;
    const USER = libc::CLONE_NEWUSER as u32;
    const IPC = libc::CLONE_NEWIPC as u32 ;
    const CGROUP = libc::CLONE_NEWCGROUP as u32;
    const UTS = libc::CLONE_NEWUTS as u32;
  }
}

enum UnshareError {
    InvalidArgument,
    NoMemory,
    PermissionDenied,
}

fn unshare(namespaces: Namespaces) -> Result<(), UnshareError> {
    unsafe {
        let exit_code = libc::unshare(namespaces.bits as i32);
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
