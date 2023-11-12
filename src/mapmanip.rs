use crate::error::Result;

byond_fn!(fn mapmanip_load_map(path_dmm, path_config) {
    load_map_guard(path_dmm, path_config).ok()
});

fn load_map_guard(path_dmm: &str, path_config: &str) -> Result<String> {
    let loaded = load_map(path_dmm, path_config);
    return if loaded.is_err() {
        crate::file::read(path_dmm)
    } else {
        loaded
    };
}

fn load_map(path_dmm: &str, path_config: &str) -> Result<String> {
    let dmm = crate::file::read(path_dmm)?;
    let _json = crate::file::read(path_config)?;
    Ok(dmm)
}
