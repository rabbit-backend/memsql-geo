use geo::{BooleanOps, Contains, Geometry, Intersects, Polygon, Rect, coord};
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

    fn st_clip_bbox(bbox: String, geom: String) -> String {
        _st_clip_bbox(bbox, geom)
    }
}

fn _st_clip_bbox(bbox: String, geom: String) -> String {
    match (
        Geometry::<f64>::try_from_wkt_str(&bbox),
        Geometry::<f64>::try_from_wkt_str(&geom),
    ) {
        (Ok(Geometry::Polygon(bbox)), geom) => match geom {
            Ok(Geometry::Polygon(geom)) => geom.intersection(&bbox).to_wkt().to_string(),
            Ok(Geometry::LineString(geom)) => Polygon::new(geom, vec![])
                .intersection(&bbox)
                .to_wkt()
                .to_string(),
            _ => "".to_owned(),
        },
        _ => "".to_owned(),
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
            let (min_x, min_y, max_x, max_y) =
                (bounds.left, bounds.bottom, bounds.right, bounds.top);

            Rect::new(coord! {x: min_x, y: min_y}, coord! {x: max_x, y: max_y})
                .to_polygon()
                .to_wkt()
                .to_string()
        }
        Err(_) => "".to_string(),
    }
}
