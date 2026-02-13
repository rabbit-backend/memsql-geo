use geo::{BooleanOps, Centroid, Contains, Geometry, Intersects, Polygon, Rect, coord};
use h3o::{LatLng, Resolution};
use tile_grid::{Xyz, tms};
use wkt::{ToWkt, TryFromWkt};

wit_bindgen_rust::export!("memsql-geo.wit");

struct MemsqlGeo;

impl memsql_geo::MemsqlGeo for MemsqlGeo {
    fn st_tile_envelope(z: u8, x: u64, y: u64) -> String {
        _st_tile_envelope(z, x, y)
    }

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

    fn st_centroid(geom: String) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geom).ok() {
            Some(geom) => geom
                .centroid()
                .map(|point| point.wkt_string())
                .unwrap_or("".to_string()),
            None => "".to_string(),
        }
    }

    fn st_h3_cell(geom: String, resolution: u8) -> String {
        match Geometry::<f64>::try_from_wkt_str(&geom).ok() {
            None => "".to_string(),
            Some(Geometry::Point(point)) => match LatLng::new(point.x(), point.y()) {
                Ok(point) => match resolution {
                    1 => point.to_cell(Resolution::One).to_string(),
                    2 => point.to_cell(Resolution::Two).to_string(),
                    3 => point.to_cell(Resolution::Three).to_string(),
                    4 => point.to_cell(Resolution::Four).to_string(),
                    5 => point.to_cell(Resolution::Five).to_string(),
                    6 => point.to_cell(Resolution::Six).to_string(),
                    7 => point.to_cell(Resolution::Seven).to_string(),
                    8 => point.to_cell(Resolution::Eight).to_string(),
                    9 => point.to_cell(Resolution::Nine).to_string(),
                    10 => point.to_cell(Resolution::Ten).to_string(),
                    11 => point.to_cell(Resolution::Eleven).to_string(),
                    12 => point.to_cell(Resolution::Twelve).to_string(),
                    13 => point.to_cell(Resolution::Thirteen).to_string(),
                    14 => point.to_cell(Resolution::Fourteen).to_string(),
                    15 => point.to_cell(Resolution::Fifteen).to_string(),
                    _ => "".to_string(),
                },
                Err(_) => "".to_string(),
            },
            _ => "".to_string(),
        }
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
            let bounds = tms.bounds(&Xyz::new(x, y, z));

            match bounds {
                Ok(bounds) => {
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
        Err(_) => "".to_string(),
    }
}
