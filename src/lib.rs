use geo::polygon;
use tile_grid::{Xyz, tms};
use wkt::ToWkt;
wit_bindgen_rust::export!("memsql-geo.wit");

struct MemsqlGeo;

impl memsql_geo::MemsqlGeo for MemsqlGeo {
    fn st_tile_envelope(z: u8, x: u64, y: u64) -> String {
        _st_tile_envelope(z, x, y)
    }

    fn st_as_mvt_geom() -> String {
        "".to_string()
    }
}

fn _st_tile_envelope(z: u8, x: u64, y: u64) -> String {
    // SRID 3857
    match tms().lookup("WebMercatorQuad") {
        Ok(tms) => {
            let bounds = tms.xy_bounds(&Xyz::new(x, y, z));
            let (x1, y1, x2, y2) = (bounds.bottom, bounds.left, bounds.top, bounds.right);

            polygon![
                (x: y1, y: x1),
                (x: y2, y: x1),
                (x: y2, y: x2),
                (x: y1, y: x2),
            ]
            .to_wkt()
            .to_string()
        }
        Err(_) => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_tile_envelope() {
        println!(
            "{}\n{}",
            _st_tile_envelope(10, 10, 14),
            "POLYGON ((-19646150.75796914 19450471.96555909, -19646150.75796914 19489607.7240411, -19607014.99948713 19489607.7240411, -19607014.99948713 19450471.96555909, -19646150.75796914 19450471.96555909))"
        )
    }
}
