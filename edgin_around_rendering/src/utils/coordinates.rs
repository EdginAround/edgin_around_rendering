pub fn degrees(radians: f32) -> f32 {
    180.0 * radians * std::f32::consts::FRAC_1_PI
}

pub fn radians(degrees: f32) -> f32 {
    const ONE_OVER_180: f32 = 1.0 / 180.0;
    degrees * ONE_OVER_180 * std::f32::consts::PI
}

pub fn cartesian_to_spherical(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;
    let r = (x * x + y * y + z * z).sqrt();
    let theta = if y != 0.0 { (x * x + z * z).sqrt().atan2(y) } else { HALF_PI };
    let phi = if z != 0.0 { x.atan2(z) } else { HALF_PI };
    (r, theta, phi)
}

pub fn cartesian_to_geographical_radians(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let (r, theta, phi) = cartesian_to_spherical(x, y, z);
    spherical_to_geographical_radians(r, theta, phi)
}

pub fn cartesian_to_geographical_degrees(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let (r, theta, phi) = cartesian_to_spherical(x, y, z);
    spherical_to_geographical_degrees(r, theta, phi)
}

pub fn spherical_to_cartesian(r: f32, theta: f32, phi: f32) -> (f32, f32, f32) {
    let z = r * theta.sin() * phi.cos();
    let x = r * theta.sin() * phi.sin();
    let y = r * theta.cos();
    (x, y, z)
}

pub fn spherical_to_geographical_radians(r: f32, theta: f32, phi: f32) -> (f32, f32, f32) {
    let lat = 0.5 * std::f32::consts::PI - theta;
    let lon = if phi <= std::f32::consts::PI { phi } else { phi - 2.0 * std::f32::consts::PI };
    (r, lat, lon)
}

pub fn spherical_to_geographical_degrees(r: f32, theta: f32, phi: f32) -> (f32, f32, f32) {
    let (r, lat, lon) = spherical_to_geographical_radians(r, theta, phi);
    (r, degrees(lat), degrees(lon))
}

pub fn geographical_radians_to_spherical(r: f32, lat: f32, lon: f32) -> (f32, f32, f32) {
    let theta = std::f32::consts::FRAC_PI_2 - lat;
    let phi = if lon >= 0.0 { lon } else { lon + 2.0 * std::f32::consts::PI };
    (r, theta, phi)
}

pub fn geographical_degrees_to_spherical(r: f32, lat: f32, lon: f32) -> (f32, f32, f32) {
    geographical_radians_to_spherical(r, radians(lat), radians(lon))
}

/// Position expressed in geographical coordinates with radians.
pub struct Coordinate {
    lat: f32,
    lon: f32,
}

impl Coordinate {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }

    pub fn moved_by(&self, distance: f32, bearing: f32, radius: f32) -> Coordinate {
        let angular_distance = distance / radius;
        let cad = angular_distance.cos();
        let sad = angular_distance.sin();

        let cb = bearing.cos();
        let sb = bearing.sin();

        let slat1 = self.lat.sin();
        let clat1 = self.lat.cos();

        let lat2 = (slat1 * cad + clat1 * sad * cb).asin();
        let slat2 = lat2.sin();
        let lon2 = self.lon + (sb * sad * clat1).atan2(cad - slat1 * slat2);

        Coordinate::new(lat2, lon2)
    }

    pub fn to_point(&self) -> Point {
        let (_r, theta, phi) = geographical_radians_to_spherical(1.0, self.lat, self.lon);
        Point::new(theta, phi)
    }
}

/// Position expressed in spherical coordinates.
#[derive(Clone, Debug)]
pub struct Point {
    pub theta: f32,
    pub phi: f32,
}

impl Point {
    pub fn new(theta: f32, phi: f32) -> Self {
        Self { theta, phi }
    }
}

impl Point {
    pub fn moved_by(&self, distance: f32, bearing: f32, radius: f32) -> Point {
        self.to_coordinate().moved_by(distance, bearing, radius).to_point()
    }

    pub fn to_coordinate(&self) -> Coordinate {
        let (_r, lat, lon) = spherical_to_geographical_radians(1.0, self.theta, self.phi);
        Coordinate::new(lat, lon)
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub theta: f32,
    pub phi: f32,
    pub bearing: f32,
    pub altitude: f32,
}

impl Position {
    pub fn new(theta: f32, phi: f32, bearing: f32, altitude: f32) -> Self {
        Self { theta, phi, bearing, altitude }
    }
}

#[derive(Clone, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn stretch(&mut self, multiplier: f32) {
        self.x *= multiplier;
        self.y *= multiplier;
        self.z *= multiplier;
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn enlongated(&self, new_length: f32) -> Self {
        let factor = new_length / self.length();
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl std::ops::Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Add for &Point3D {
    type Output = Point3D;

    fn add(self, other: &Point3D) -> Point3D {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
