use std::{
    error::Error,
    fmt::{write, Display},
};

use bitflags::bitflags;
use nix::{errno, libc};
use oci_spec::runtime::{LinuxNamespace, LinuxNamespaceType};

bitflags! {
  pub struct CloneFlags: u32 {
    /// Network Namespace for isolating network devices, ports, stacks etc.
    const NET = LinuxNamespaceType::Network as u32;
    /// PID Namespace for isolating process ids
    const PID = LinuxNamespaceType::Pid as u32;
    /// Mount Namespace for isolating mount points
    const MOUNT = LinuxNamespaceType::Mount as u32;
    /// User Namespace for isolating user and group  ids
    const USER = LinuxNamespaceType::User as u32;
    /// Ipc Namespace for isolating System V, IPC, POSIX message queues
    const IPC = LinuxNamespaceType::Ipc as u32 ;
    /// Cgroup Namespace for isolating cgroup hierarchies
    const CGROUP = LinuxNamespaceType::Cgroup as u32;
    /// Uts Namespace for isolating hostname and NIS domain name
    const UTS = LinuxNamespaceType::Uts as u32;
  }
}

fn get_clone_flag(namespace: &LinuxNamespace) -> CloneFlags {
    match namespace.typ() {
        LinuxNamespaceType::Mount => CloneFlags::MOUNT,
        LinuxNamespaceType::Cgroup => CloneFlags::CGROUP,
        LinuxNamespaceType::Uts => CloneFlags::UTS,
        LinuxNamespaceType::Ipc => CloneFlags::IPC,
        LinuxNamespaceType::User => CloneFlags::USER,
        LinuxNamespaceType::Pid => CloneFlags::PID,
        LinuxNamespaceType::Network => CloneFlags::NET,
    }
}

impl From<&Option<Vec<LinuxNamespace>>> for CloneFlags {
    fn from(ns: &Option<Vec<LinuxNamespace>>) -> Self {
        let mut clone_flags = CloneFlags::empty();
        ns.as_ref().unwrap_or(&vec![])
            .iter()
            .map(|ns| (get_clone_flag(ns)))
            .for_each(|clone_flag| clone_flags.insert(clone_flag));

        clone_flags
    }
}

impl From<&LinuxNamespace> for CloneFlags {
    fn from(ns: &LinuxNamespace) -> Self {
        get_clone_flag(ns)
    }
}

#[derive(Debug)]
pub enum UnshareError {
    InvalidArgument,
    NoMemory,
    PermissionDenied,
}

impl Display for UnshareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnshareError::InvalidArgument => write!(f, "invalid argument"),
            UnshareError::NoMemory => write!(f, "no memory left on device"),
            UnshareError::PermissionDenied => write!(f, "permission denied"),
        }
    }
}
impl Error for UnshareError {}

pub fn unshare(clone_flags: CloneFlags) -> Result<(), UnshareError> {
    unsafe {
        let exit_code = libc::unshare(clone_flags.bits() as i32);
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

#[cfg(test)]
mod tests {

    use oci_spec::runtime::LinuxNamespaceBuilder;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    fn gen_namespace_vec() -> Vec<LinuxNamespace> {
        vec![
            LinuxNamespaceBuilder::default()
                .typ(LinuxNamespaceType::Network)
                .build()
                .unwrap(),
            LinuxNamespaceBuilder::default()
                .typ(LinuxNamespaceType::Cgroup)
                .build()
                .unwrap(),
            LinuxNamespaceBuilder::default()
                .typ(LinuxNamespaceType::Mount)
                .build()
                .unwrap(),
        ]
    }
    #[test]
    fn test_clone_flags_from() {
        let mut expected_clone_flags = CloneFlags::empty();
        expected_clone_flags.insert(CloneFlags::MOUNT);
        expected_clone_flags.insert(CloneFlags::CGROUP);
        expected_clone_flags.insert(CloneFlags::NET);

        let spec_namespaces = gen_namespace_vec();
        let actual_clone_flags = CloneFlags::from(&Some(spec_namespaces));

        assert_eq!(expected_clone_flags, actual_clone_flags)
    }
}
