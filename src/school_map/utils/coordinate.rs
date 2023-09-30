use super::super::structs::{Params, GeoCoord, ImgCoord};

pub fn img_to_geo(params: &Params, xy: ImgCoord) -> GeoCoord {
    let x = xy[0] as f64;
    let y = xy[1] as f64;
    let [a, b] = &params.base;
    let [c1, c2] = &params.c12;
    let theta = &params.theta_deg * (std::f64::consts::PI / 180.0);
    let lon = c1 * (theta.cos() * x + theta.sin() * y) + a;
    let lat = c2 * (theta.sin() * x - theta.cos() * y) + b;
    [lon, lat]
}

pub fn geo_to_img(params: &Params, lonlat: GeoCoord) -> ImgCoord {
    let [a, b] = &params.base;
    let [c3, c4] = &params.c34;
    let theta = &params.theta_deg * (std::f64::consts::PI / 180.0);
    let lon = lonlat[0] - a;
    let lat = lonlat[1] - b;
    let x = c3 * (theta.cos() * lon + theta.sin() * lat);
    let y = c4 * (theta.sin() * lon - theta.cos() * lat);
    let x = x as u64;
    let y = y as u64;
    [x, y]
}
