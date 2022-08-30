use std::error::Error;

use oci_spec::runtime;

use crate::{
    container::fork,
    namespace::{unshare, CloneFlags},
};

pub struct IntermediateProcess<'a> {
    pub spec: &'a runtime::Spec,
    pub clone_flags: &'a CloneFlags,
}

impl IntermediateProcess<'_> {
    pub fn container_intermediate_process(&self) -> Result<(), Box<dyn Error>> {
        println!("hello from child");
        if self.clone_flags.contains(CloneFlags::USER) {
            unshare(CloneFlags::USER)?;
            // TODO: send request for uid mappings write
        }
        if self.clone_flags.contains(CloneFlags::PID) {
            unshare(CloneFlags::PID)?;
        }

        fork(
            |child| {
                // TODO: send ready with init process id
                Ok(())
            },
            || {
                unshare(
                    self.clone_flags
                        .difference(CloneFlags::PID | CloneFlags::USER),
                )?;
                // TODO: setup mounts
                // TODO: setup hostname
                // TODO: setup user
                // TODO: set capabilities

                Ok(())
            },
        )
    }
}
