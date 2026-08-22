#![allow(dead_code)]
#![allow(unused_variables)]

use crate::engine::math::vec3::Vec3;

#[derive(Clone)]
pub struct ConvexHull {
    vertices: Vec<Vec3>,
}
impl ConvexHull {
    pub fn support(&self, dir: Vec3) -> Vec3 {
        self.vertices
            .iter()
            .copied()
            .max_by(|a, b| a.dot(dir).partial_cmp(&b.dot(dir)).unwrap())
            .unwrap()
    }
}

fn support_minkowski(a: ConvexHull, b: ConvexHull, dir: Vec3) -> Vec3 {
    a.support(dir) - b.support(Vec3{x: -dir.x, y: -dir.y, z: -dir.z})
}

fn same_direction(a: Vec3, b: Vec3) -> bool {
    a.dot(b) > 0.0
}

fn triple_product(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    a.cross(b).cross(c)
}

fn line_case(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool {
    let a = simplex[1]; // newest
    let b = simplex[0];
    let ab = b - a;
    let ao: Vec3 = -a;

    if same_direction(ab, ao) {
        *dir = triple_product(ab, ao, ab);
        if dir.length_squared() < f32::EPSILON {
            let fallback = if ab.cross(Vec3{x: 1.0, y: 0.0, z: 0.0}).length_squared() > f32::EPSILON {Vec3{x: 1.0, y: 0.0, z: 0.0} } else { Vec3{x: 0.0, y: 1.0, z: 0.0} };
            *dir = ab.cross(fallback);
        }
    } else {
        *simplex = vec![a];
        *dir = ao;
    }
    false
}

fn triangle_case(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool {
    let a = simplex[2]; // newest
    let b = simplex[1];
    let c = simplex[0];

    let ab = b - a;
    let ac = c - a;
    let ao: Vec3 = -a;
    let abc = ab.cross(ac);

    if same_direction(abc.cross(ac), ao) {
        if same_direction(ac, ao) {
            *simplex = vec![c, a];
            *dir = triple_product(ac, ao, ac);
        } else {
            *simplex = vec![b, a];
            return line_case(simplex, dir);
        }
    } else if same_direction(ab.cross(abc), ao) {
        *simplex = vec![b, a];
        return line_case(simplex, dir);
    } else if same_direction(abc, ao) {
        *dir = abc;
    } else {
        *simplex = vec![b, c, a];
        *dir = -abc;
    }
    false
}

fn tetrahedron_case(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool {
    let a = simplex[3]; // newest
    let b = simplex[2];
    let c = simplex[1];
    let d = simplex[0];

    let ab = b - a;
    let ac = c - a;
    let ad = d - a;
    let ao: Vec3 = -a;

    let abc = ab.cross(ac);
    let acd = ac.cross(ad);
    let adb = ad.cross(ab);

    if same_direction(abc, ao) {
        *simplex = vec![c, b, a];
        return triangle_case(simplex, dir);
    }
    if same_direction(acd, ao) {
        *simplex = vec![d, c, a];
        return triangle_case(simplex, dir);
    }
    if same_direction(adb, ao) {
        *simplex = vec![b, d, a];
        return triangle_case(simplex, dir);
    }

    true
}

fn do_simplex(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool {
    match simplex.len() {
        2 => line_case(simplex, dir),
        3 => triangle_case(simplex, dir),
        4 => tetrahedron_case(simplex, dir),
        _ => unreachable!("simplex should only ever have 2-4 points here"),
    }
}

pub fn gjk_intersect(a: ConvexHull, b: ConvexHull) -> bool {
    let mut dir = Vec3{x: 1.0, y: 0.0, z: 0.0};
    let mut simplex: Vec<Vec3> = vec![support_minkowski(a.clone(), b.clone(), dir)];
    dir = Vec3{x: -1.0, y: -1.0, z: -1.0} * simplex[0];

    loop {
        let point = support_minkowski(a.clone(), b.clone(), dir);
        if point.dot(dir) < 0.0 {
            return false;
        }
        simplex.push(point);

        if do_simplex(&mut simplex, &mut dir) {
            return true;
        }
    }
}