use rand::Rng;

pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub continent: &'static str,
}

fn generate_random_point(
    lat_range: (f64, f64),
    lon_range: (f64, f64),
    continent: &'static str,
) -> Coordinate {
    let mut rng = rand::thread_rng();
    let latitude = rng.gen_range(lat_range.0..=lat_range.1);
    let longitude = rng.gen_range(lon_range.0..=lon_range.1);
    Coordinate {
        latitude,
        longitude,
        continent,
    }
}

pub fn generate_points_through_continents(amount: usize) -> Vec<Coordinate> {
    // Latitude and longitude boundaries for each continent
    let continents = [
        ("Africa", (-35.0, 37.0), (-17.0, 51.0)),
        ("Asia", (5.0, 77.0), (26.0, 169.0)),
        ("Europe", (35.0, 71.0), (-10.0, 60.0)),
        ("North America", (7.0, 83.0), (-172.0, -53.0)),
        ("South America", (-55.0, 13.0), (-81.0, -35.0)),
        ("Australia", (-50.0, -10.0), (110.0, 180.0)),
        ("Antarctica", (-90.0, -60.0), (-180.0, 180.0)),
    ];

    // Generate 100 coordinates alternating through the continents
    let mut coordinates = Vec::new();
    for i in 0..amount {
        let (continent, lat_range, lon_range) = continents[i % continents.len()];
        let point = generate_random_point(lat_range, lon_range, continent);
        coordinates.push(point);
    }

    coordinates
}
