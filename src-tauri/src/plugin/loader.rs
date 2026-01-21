use libloading::Library;

use crate::{app::host::NovaHost, plugin::api::PluginInit};

pub struct Plugin {
    _lib: Library,
}

impl Plugin {
    pub unsafe fn load(path: &str, host: &NovaHost) -> anyhow::Result<Self> {
        let lib = Library::new(path)?;
        let init: libloading::Symbol<PluginInit> = lib.get(b"novacl_plugin_init")?;
        init(host as *const _);
        Ok(Self { _lib: lib })
    }
}
