use std::error::Error;

use nix::unistd::Pid;
use oci_spec::runtime;

use crate::{container::mappings, namespace::CloneFlags};

pub struct MainProcess<'a> {
    pub child: Pid,
    pub spec: &'a runtime::Spec,
    pub clone_flags: &'a CloneFlags,
}
impl MainProcess<'_> {
    pub fn container_main_process(&self) -> Result<(), Box<dyn Error>> {
        let linux_spec = self.spec.linux().as_ref().unwrap();
        println!("started child on pid: {}, ", self.child);
        if self.clone_flags.contains(CloneFlags::USER) {
            /*
            TODO: wait for child to unshare usernamespace
            */
            if let Some(uid_mappings) = linux_spec.uid_mappings() {
                mappings::apply_id_mappings("newuidmap", self.child, uid_mappings)?;
            }
            if let Some(gid_mappings) = linux_spec.gid_mappings() {
                mappings::apply_id_mappings("newgidmap", self.child, gid_mappings)?;
            }
        }
        Ok(())
    }
}
