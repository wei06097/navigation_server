use crate::school_map::{self, structs::Params};

#[test]
fn read_json() {
    let path = vec!["src", "school_map", "tests", "assets", "params.json"];
    let params = school_map::read_json::<Params>(&path).unwrap();
    assert_eq!(params.theta_deg, 0.0);
    assert_eq!(params.base, [121.548763, 25.012345]);
    assert_eq!(params.c12, [1.1, 1.2]);
    assert_eq!(params.c34, [2.1, 2.2]);
}

mod coordinate {
    use crate::school_map::{coordinate, structs::Params};
    
    #[test]
    fn coordinate_transform() {
        let params = Params {
            theta_deg: -90.0,
            base: [1.0, 1.0],
            c12: [1.0, 1.0],
            c34: [1.0, 1.0],
        };
        let data_xy = [1, 2];
        let data_lonlat = [-1.0, 0.0];

        let temp_lonlat = coordinate::img_to_geo(&params, data_xy);
        let output_xy = coordinate::geo_to_img(&params, temp_lonlat);
        assert_eq!(output_xy, data_xy);    
        let output_xy = coordinate::geo_to_img(&params, data_lonlat);
        assert_eq!(output_xy, data_xy);
    }
    
    #[test]
    fn haversine_formula() {
        let location_a = [121.54026281917675, 25.012364685321245];
        let location_b = [121.5404747956079, 25.012529348340735];
        let distance = coordinate::haversine_formula(location_a, location_b);
        let distance = distance.round() as u64;
        assert_eq!(distance, 28);
    }
}
