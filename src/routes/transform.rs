use crate::routes;
use crate::school_map::{
    self, coordinate,
    structs::{Params, GeoCoord, ImgCoord},
};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

/// 定義座標可能的資料型態
#[derive(Debug, Deserialize, Serialize)]
pub enum CoordType {
    Geo(GeoCoord),
    Img(ImgCoord),
}
/// 定義 Body 結構
#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    coord: CoordType,
}

#[post("/transform")]
pub async fn transform(data: web::Json<Body>) -> impl Responder {
    let path = routes::PARAMS_FILEPATH.to_vec();
    let params = school_map::read_json::<Params>(&path).unwrap();
    let coord = match data.coord {
        CoordType::Geo(coord) => CoordType::Img(coordinate::geo_to_img(&params, coord)),
        CoordType::Img(coord) => CoordType::Geo(coordinate::img_to_geo(&params, coord)),
    };
    HttpResponse::Ok().json(Body {
        coord
    })
}
