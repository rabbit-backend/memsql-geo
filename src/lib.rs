use geo::{Contains, Geometry, Intersects, Rect, coord};
use tile_grid::{Xyz, tms};
use wkt::{ToWkt, TryFromWkt};
wit_bindgen_rust::export!("memsql-geo.wit");

struct MemsqlGeo;

impl memsql_geo::MemsqlGeo for MemsqlGeo {
    fn st_tile_envelope(z: u8, x: u64, y: u64) -> String {
        _st_tile_envelope(z, x, y)
    }

    fn st_intersects(geom1: String, geom2: String) -> bool {
        _st_intersects(geom1, geom2)
    }

    fn st_contains(geom1: String, geom2: String) -> bool {
        _st_contains(geom1, geom2)
    }
}

fn _st_contains(geom1: String, geom2: String) -> bool {
    match (
        Geometry::<f64>::try_from_wkt_str(&geom1),
        Geometry::<f64>::try_from_wkt_str(&geom2),
    ) {
        (Ok(geom1), Ok(geom2)) => geom1.contains(&geom2),
        _ => false,
    }
}

fn _st_intersects(geom1: String, geom2: String) -> bool {
    match (
        Geometry::<f64>::try_from_wkt_str(&geom1),
        Geometry::<f64>::try_from_wkt_str(&geom2),
    ) {
        (Ok(geom1), Ok(geom2)) => geom1.intersects(&geom2),
        _ => false,
    }
}

fn _st_tile_envelope(z: u8, x: u64, y: u64) -> String {
    match tms().lookup("WebMercatorQuad") {
        Ok(tms) => {
            let bounds = tms.xy_bounds(&Xyz::new(x, y, z));
            let (x1, y1, x2, y2) = (bounds.left, bounds.bottom, bounds.right, bounds.top);

            Rect::new(coord! {x: x1, y: y1}, coord! {x: x2, y: y2})
                .to_polygon()
                .to_wkt()
                .to_string()
        }
        Err(_) => "".to_string(),
    }
}
