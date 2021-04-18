// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::f32::consts::PI;

use edgin_around_rendering::{
    game::{Actor, Scene},
    utils::coordinates::Point,
};

#[test]
fn find_closest_actors() {
    let mut scene = Scene::new();
    scene.create_actors(&vec![
        Actor::new(0, "0".to_string(), Some(Point::new(0.00 * PI, 0.01 * PI))),
        Actor::new(1, "1".to_string(), Some(Point::new(0.05 * PI, 0.05 * PI))),
        Actor::new(2, "2".to_string(), None),
        Actor::new(3, "3".to_string(), Some(Point::new(0.01 * PI, 0.02 * PI))),
        Actor::new(4, "4".to_string(), Some(Point::new(0.06 * PI, 0.06 * PI))),
        Actor::new(5, "5".to_string(), Some(Point::new(0.02 * PI, 0.03 * PI))),
    ]);

    let expected = vec![0, 3, 5];
    let actual = scene.find_closest_actors(&Point::new(0.0, 0.0), 100.0);

    assert_eq!(actual, expected);
}
