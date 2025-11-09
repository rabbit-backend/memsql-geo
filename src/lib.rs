use geo::{BooleanOps, Geometry, Rect, coord};
use tile_grid::{Xyz, tms};
use wkt::{ToWkt, TryFromWkt};
wit_bindgen_rust::export!("memsql-geo.wit");

struct MemsqlGeo;

impl memsql_geo::MemsqlGeo for MemsqlGeo {
    fn st_tile_envelope(z: u8, x: u64, y: u64) -> String {
        _st_tile_envelope(z, x, y)
    }

    fn st_as_mvt_geom(geom: String, bbox: String) -> String {
        match Rect::<f64>::try_from_wkt_str(&bbox) {
            Ok(bbox) => match Geometry::<f64>::try_from_wkt_str(&geom).ok() {
                Some(Geometry::Polygon(polygon)) => {
                    // clip the geometry with the bounding box
                    let clipped_polygon = &bbox.to_polygon().intersection(&polygon);
                    clipped_polygon.to_wkt().to_string()
                }
                Some(Geometry::LineString(ls)) => ls.to_wkt().to_string(),
                Some(Geometry::Point(point)) => point.to_wkt().to_string(),
                None => "".to_string(),
                _ => "".to_string(),
            },
            Err(_) => "".to_string(),
        }
    }
}

fn _st_tile_envelope(z: u8, x: u64, y: u64) -> String {
    // SRID 3857
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
