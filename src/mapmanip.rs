use crate::error::Result;
use rand::seq::SliceRandom;

byond_fn!(fn mapmanip_load_map(path_dmm) {
    load_map_guard(path_dmm).ok()
});

fn load_map_guard(path_dmm: &str) -> Result<String> {
    let path_config = path_dmm.replace(".dmm", ".toml");

    let loaded = load_map(path_dmm, &path_config);
    return if loaded.is_err() {
        crate::file::read(path_dmm)
    } else {
        loaded
    };
}

fn load_map(path_dmm: &str, path_config: &str) -> Result<String> {
    let dmm_raw = crate::file::read(path_dmm)?;
    let _config_raw = crate::file::read(path_config)?;

    let dmm_parsed = dmm_parser_rs::dmmr::parse(&dmm_raw);

    let mut dmm_unpacked = dmm_parser_rs::dmmr::unpack(&dmm_parsed);

    let src_coord = [(60, 40), (65, 40), (70, 40), (60, 35), (65, 35), (70, 35)]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone();
    let src_coord = (src_coord.0 - 1, src_coord.1 - 1);
    let dst_coord = (65 - 1, 45 - 1);
    let size = (4, 4);

    let extracted =
        dmm_parser_rs::copypaste::extract(&dmm_unpacked, (src_coord).into(), size.into());
    dmm_parser_rs::copypaste::insert(&extracted, &mut dmm_unpacked, dst_coord.into());

    let dmm_parsed = dmm_parser_rs::dmmr::pack(&dmm_unpacked);

    Ok(dmm_parser_rs::dmmr::print(&dmm_parsed))
}
