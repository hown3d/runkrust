use std::{error::Error, fmt::Display};

use nix::{
    sys::wait::waitpid,
    unistd::{self, setuid, ForkResult, Pid, Uid},
};
use oci_spec::runtime::{self, LinuxNamespace};
use std::io::prelude::*;

use crate::namespace::{self, unshare, CloneFlags, UnshareError};

use self::processes::{main_process::MainProcess, intermediate_process::{self, IntermediateProcess}};

mod mappings;
mod processes;

#[derive(Debug)]
pub enum ContainerError {
    InvalidConfiguration(String),
}

impl Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerError::InvalidConfiguration(s) => write!(f, "Invalid Configuration: {}", s),
        }
    }
}

impl Error for ContainerError {}

pub fn create(id: &str, spec: runtime::Spec) -> Result<(), Box<dyn Error>> {
    let linux_spec = spec.linux();
    if linux_spec.is_none() {
        return Err(ContainerError::InvalidConfiguration(String::from(
            "spec.linux can't be empty",
        ))
        .into());
    }

    let linux_spec = linux_spec.as_ref().unwrap();

    let clone_flags = namespace::CloneFlags::from(linux_spec.namespaces());

    fork(
        |child| {
            let main_process = MainProcess {
                child,
                spec: &spec,
                clone_flags: &clone_flags,
            };
            main_process.container_main_process()
        },
        || {
            let intermediate_process = IntermediateProcess {
                spec: &spec,
                clone_flags: &clone_flags,
            };
            intermediate_process.container_intermediate_process()
        },
    )?;

    Ok(())
}

fn fork<F, T>(parent_func: F, child_func: T) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(Pid) -> Result<(), Box<dyn Error>>,
    T: FnOnce() -> Result<(), Box<dyn Error>>,
{
    let pid = unsafe { unistd::fork() }?;
    match pid {
        ForkResult::Parent { child, .. } => {
            parent_func(child)
            // waitpid(child, None).unwrap();
        }
        ForkResult::Child => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            // write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
            // unsafe { libc::_exit(0) };
            child_func()
        }
    }
}

fn start(spec: runtime::Spec) {}

fn delete(spec: runtime::Spec) {}

fn kill(spec: runtime::Spec) {}
