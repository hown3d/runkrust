use libocispec::runtime::{self, Spec};

pub fn create(id: &str, spec: runtime::Spec) {
    panic!("ContainerID: {}, hello", id)
}

fn start(spec: runtime::Spec) {}

fn delete(spec: runtime::Spec) {}

fn kill(spec: runtime::Spec) {}
