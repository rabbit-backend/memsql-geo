use geo::{BooleanOps, Contains, Geometry, Intersects, Polygon, Rect, coord};
use wkt::{ToWkt, TryFromWkt};
wit_bindgen_rust::export!("memsql-geo.wit");

struct MemsqlGeo;

impl memsql_geo::MemsqlGeo for MemsqlGeo {
    fn st_parse_bbox(bbox: String) -> String {
        match bbox.split(',').collect::<Vec<_>>().as_slice() {
            [x_min, y_min, x_max, y_max] => match (
                x_min.parse::<f64>(),
                y_min.parse::<f64>(),
                x_max.parse::<f64>(),
                y_max.parse::<f64>(),
            ) {
                (Ok(x_min), Ok(y_min), Ok(x_max), Ok(y_max)) => {
                    Rect::new(coord! {x: x_min, y: y_min}, coord! {x: x_max, y: y_max})
                        .to_polygon()
                        .to_wkt()
                        .to_string()
                }
                _ => "".to_owned(),
            },
            _ => "".to_owned(),
        }
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
