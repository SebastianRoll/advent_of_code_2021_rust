use std::cmp;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector {
    x: isize,
    y: isize,
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vector {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn drag(self) -> Vector {
        //
        let x_new = self.x.signum() * (self.x.abs() - 1);
        Self {
            x: x_new,
            y: self.y - 1,
        }
    }
}

struct Rectangle {
    ul: Vector,
    lr: Vector,
}

impl Rectangle {
    fn new(ul: Vector, lr: Vector) -> Self {
        Self { ul, lr }
    }
    fn inside(&self, pos: Vector) -> bool {
        pos.x >= self.ul.x && pos.y <= self.ul.y && pos.x <= self.lr.x && pos.y >= self.lr.y
    }
}

/// Returns true if probe at position `pos` travelling at velocity `vel` hits the target area `rect`
fn is_hit(vel: Vector, pos: Vector, rect: &Rectangle) -> bool {
    let mut vel = vel;
    let mut pos = pos;
    let mut highest_y = 0;
    loop {
        pos = pos + vel;
        highest_y = cmp::max(highest_y, pos.y);
        if rect.inside(pos) {
            // this print is only for part1
            eprintln!("highest_y = {:?}", highest_y);
            return true;
        } else if pos.x > rect.lr.x || pos.y < rect.lr.y {
            return false;
        } else {
            vel = vel.drag();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_positive_x() {
        let vel = Vector::new(10, 0);
        assert_eq!(vel.drag(), Vector::new(9, -1));
    }

    #[test]
    fn test_drag_negative_x() {
        let vel = Vector::new(-10, 0);
        assert_eq!(vel.drag(), Vector::new(-9, -1));
    }

    #[test]
    fn test_rectangle_is_inside() {
        let rect = Rectangle::new(Vector::new(0, 5), Vector::new(10, 0));
        assert_eq!(rect.inside(Vector::new(10, 3)), true);
        assert_eq!(rect.inside(Vector::new(5, 3)), true);
        assert_eq!(rect.inside(Vector::new(6, 3)), true);
    }

    #[test]
    fn test_rectangle_is_outside() {
        let rect = Rectangle::new(Vector::new(0, 5), Vector::new(10, 0));
        assert_eq!(rect.inside(Vector::new(11, 3)), false);
        assert_eq!(rect.inside(Vector::new(-1, 3)), false);
    }

    #[test]
    fn test_shot_hits() {
        // target area: x=20..30, y=-10..-5
        let rect = Rectangle::new(Vector::new(20, -5), Vector::new(30, -10));
        let pos = Vector::new(0, 0);

        let vel = Vector::new(7, 2);
        let hit = is_hit(vel, pos, &rect);
        assert_eq!(hit, true);
        let vel = Vector::new(6, 3);
        let hit = is_hit(vel, pos, &rect);
        assert_eq!(hit, true);
        let vel = Vector::new(9, 0);
        let hit = is_hit(vel, pos, &rect);
        assert_eq!(hit, true);
    }

    #[test]
    fn test_shot_misses() {
        // target area: x=20..30, y=-10..-5
        let rect = Rectangle::new(Vector::new(20, -5), Vector::new(30, -10));
        let pos = Vector::new(0, 0);

        let vel = Vector::new(17, -4);
        let hit = is_hit(vel, pos, &rect);
        assert_eq!(hit, false);
    }

    #[test]
    fn test_highest_possible_shot() {
        // target area: x=244..303, y=-91..-54
        let rect = Rectangle::new(Vector::new(244, -54), Vector::new(303, -91));
        let pos = Vector::new(0, 0);

        // find x_init
        // using nth partial sum formula
        // x = n*(n+1)/2
        // n^2+n = 2x = 600
        // n^2+n -600 = 0
        // n = (-1+sqrt(1-(4*1*(-600)))/2
        // n = (-1+sqrt(2401))/2
        // n = (-1+49)/2 = 24

        // x can be 22..=32
        let x = 22;

        // highest y starting velocity is 90, found from the commented out block below
        /*
        for y in (0..1000).rev() {
            let vel = Vector::new(x, y);
            eprintln!("vel = {:?}", vel);
            let hit = is_hit(vel, pos, &rect);
            assert_eq!(hit, false, "hit found at vel: {:?}", vel);
        }
        */
        let vel = Vector::new(x, 90);
        let hit = is_hit(vel, pos, &rect);
        assert_eq!(hit, true, "hit not found at vel: {:?}", vel);
    }

    #[test]
    fn test_all_possible_shots_example() {
        // target area: x=20..30, y=-10..-5
        let rect = Rectangle::new(Vector::new(20, -5), Vector::new(30, -10));
        let pos = Vector::new(0, 0);

        let mut hits = 0;
        for x in 0..100 {
            for y in -100..100 {
                let vel = Vector::new(x, y);
                if is_hit(vel, pos, &rect) {
                    hits += 1;
                }
            }
        }
        assert_eq!(hits, 112, "hits found: {:?}", hits);
    }

    #[test]
    fn test_all_possible_shots() {
        // target area: x=244..303, y=-91..-54
        let rect = Rectangle::new(Vector::new(244, -54), Vector::new(303, -91));
        let pos = Vector::new(0, 0);

        let mut hits = 0;
        for x in 0..3000 {
            for y in -1000..1000 {
                let vel = Vector::new(x, y);
                if is_hit(vel, pos, &rect) {
                    hits += 1;
                }
            }
        }
        assert_eq!(hits, 3773, "hits found: {:?}", hits);
    }
}
