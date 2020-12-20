use std::collections::HashMap;

use crate::utils::coordinates;

type Indices3D = (u32, u32, u32);

/// Polyhedron data container.
#[derive(Clone, Debug)]
pub struct Polyhedron {
    vertices: Vec<coordinates::Point3D>,
    triangles: Vec<Indices3D>,
}

impl Polyhedron {
    pub fn new(vertices: Vec<coordinates::Point3D>, triangles: Vec<Indices3D>) -> Self {
        Self { vertices, triangles }
    }

    pub fn new_from_tuples(vertices: Vec<(f32, f32, f32)>, triangles: Vec<Indices3D>) -> Self {
        let new_vertices =
            vertices.iter().map(|t| coordinates::Point3D::new(t.0, t.1, t.2)).collect();

        Self::new(new_vertices, triangles)
    }

    pub fn get_vertices(&self) -> &Vec<coordinates::Point3D> {
        &self.vertices
    }

    pub fn get_triangles(&self) -> &Vec<Indices3D> {
        &self.triangles
    }

    pub fn rescale<S>(&mut self, stretch: S)
    where
        S: Fn(f32, f32) -> f32,
    {
        for v in self.vertices.iter_mut() {
            let (r, theta, phi) = coordinates::cartesian_to_spherical(v.x, v.y, v.z);
            v.stretch(stretch(theta, phi) / r);
        }
    }
}

/// Icosahedron generation function.
pub fn icosahedron() -> Polyhedron {
    let z = 0.0f32;
    let f = (5.0f32.sqrt() + 1.0f32) / 2.0f32;
    let b = (2.0f32 / (5.0f32 + 5.0f32.sqrt())).sqrt();
    let a = b * f;

    #[rustfmt::skip]
    Polyhedron::new_from_tuples(
        vec![( z, b, a), ( z, b,-a), ( z,-b, a), ( z,-b,-a),
             ( a, z, b), ( a, z,-b), (-a, z, b), (-a, z,-b),
             ( b, a, z), ( b,-a, z), (-b, a, z), (-b,-a, z)],
        vec![(0,  2, 4), (0,  2, 6), (1,  3,  5), (1,  3,  7),
             (4,  5, 8), (4,  5, 9), (6,  7, 10), (6,  7, 11),
             (8, 10, 0), (8, 10, 1), (9, 11,  2), (9, 11,  3),
             (4,  8, 0), (5,  8, 1), (4,  9,  2), (5,  9,  3),
             (6, 10, 0), (7, 10, 1), (6, 11,  2), (7, 11,  3)],
        )
}

/// Sphere generation function.
pub fn sphere(quality: u32, radius: f32) -> Polyhedron {
    let icosahedron = icosahedron();
    let mut vertices = icosahedron.vertices.iter().map(|v| v.enlongated(radius)).collect();
    let mut old_triangles = icosahedron.triangles;

    let mut proto_vertices = Vec::<(u32, u32)>::new();
    let mut proto_indices = HashMap::<(u32, u32), u32>::new();
    let mut new_triangles = Vec::<Indices3D>::new();

    fn index(
        index1: u32,
        index2: u32,
        vertices: &Vec<coordinates::Point3D>,
        proto_vertices: &mut Vec<(u32, u32)>,
        proto_indices: &mut HashMap<(u32, u32), u32>,
    ) -> u32 {
        let index_min = std::cmp::min(index1, index2);
        let index_max = std::cmp::max(index1, index2);
        let pair = (index_min, index_max);

        if proto_indices.contains_key(&pair) {
            *proto_indices.get(&pair).unwrap()
        } else {
            let index = (vertices.len() + proto_vertices.len()) as u32;
            proto_indices.insert(pair.clone(), index);
            proto_vertices.push(pair.clone());
            index
        }
    }

    for _i in 0..quality {
        proto_vertices.clear();
        proto_indices.clear();
        new_triangles.clear();

        for t in old_triangles.iter() {
            let p0 = index(t.1, t.2, &vertices, &mut proto_vertices, &mut proto_indices);
            let p1 = index(t.0, t.2, &vertices, &mut proto_vertices, &mut proto_indices);
            let p2 = index(t.0, t.1, &vertices, &mut proto_vertices, &mut proto_indices);

            new_triangles.push((t.0, p1, p2));
            new_triangles.push((t.1, p0, p2));
            new_triangles.push((t.2, p0, p1));
            new_triangles.push((p0, p1, p2));
        }

        for (p1, p2) in proto_vertices.iter() {
            vertices.push((&vertices[*p1 as usize] + &vertices[*p2 as usize]).enlongated(radius));
        }

        std::mem::swap(&mut old_triangles, &mut new_triangles);
    }

    Polyhedron::new(vertices, old_triangles)
}
