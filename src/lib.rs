wit_bindgen_rust::export!("memsql-geo.wit");

struct MemsqlGeo;

impl memsql_geo::MemsqlGeo for MemsqlGeo {
    fn st_tile_envelope(z: i32, x: i32, y: i32) -> String {
        todo!()
    }
}
