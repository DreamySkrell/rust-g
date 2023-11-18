use std::path::Path;

use crate::error::Result;
use rand::seq::SliceRandom;

byond_fn!(fn mapmanip_load_map(path_dmm) {
    load_map_guard(path_dmm).ok()
});

/// Try to do the manipulations, but if they fail, just read the map and do nothing else.
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
    let _config_raw = crate::file::read(path_config)?;

    let mut map = dmm_parser_rs::GridMap::from_file(Path::new(path_dmm))
        .ok_or(crate::error::Error::HexDecode)?;

    let xtr_coord = [(60, 40), (65, 40), (70, 40), (60, 35), (65, 35), (70, 35)]
        .map(|a| dmm_parser_rs::dmmtools::dmm::Coord2::new(a.0, a.1))
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone();
    let xtr_size = dmm_parser_rs::dmmtools::dmm::Coord2::new(4, 4);
    let dst_coord = dmm_parser_rs::dmmtools::dmm::Coord2::new(65, 45);

    let xtr_map = dmm_parser_rs::tools::extract_sub_map(&map, xtr_coord, xtr_size);
    dmm_parser_rs::tools::insert_sub_map(&xtr_map, dst_coord, &mut map);

    dmm_parser_rs::core::map_to_string(&dmm_parser_rs::core::to_dict_map(&map))
        .ok_or(crate::error::Error::HexDecode)
}
