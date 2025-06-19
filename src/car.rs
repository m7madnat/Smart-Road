use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lane {
    Straight,
    Right,
    Left,
    Air,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
    pub angle: Option<f64>,
}

#[derive(Clone)]
pub struct Car<'a> {
    pub id: usize,
    pub position: (f64, f64),
    pub speed: f64,
    pub waypoints: Vec<Waypoint>,
    pub lane: Lane,
    pub direction: Direction,
    pub texture: &'a Texture<'a>,
    pub angle: f64,
    pub is_waiting: bool,
    pub size: Option<(u32, u32)>,
    pub close_call_triggered: bool,
}

impl<'a> Car<'a> {
    pub fn new(
        lane: Lane,
        start: (f64, f64),
        waypoints: Vec<Waypoint>,
        speed: f64,
        id: usize,
        direction: Direction,
        texture: &'a Texture<'a>,
        size: Option<(u32, u32)>,
    ) -> Self {
        Car {
            id,
            position: start,
            speed,
            // max_reached_speed: speed,
            waypoints,
            lane,
            direction,
            texture,
            angle: match (direction, lane) {
                (_, Lane::Air) => 310.0,
                (Direction::South, _) => 360.0,
                (Direction::North, _) => 180.0,
                (Direction::East, _) => 270.0,
                (Direction::West, _) => 90.0,
            },
            is_waiting: false,
            size,
            close_call_triggered: false,
        }
    }

    pub fn update_position(&mut self, others: &[Car], close_call_counter: &mut usize) {
        if self.lane == Lane::Right || self.lane == Lane::Air {
            self.is_waiting = false;
        } else if self.is_car_in_front(others, 60.0) {
            self.speed = 0.0;
            self.is_waiting = true;

            if !self.close_call_triggered {
                *close_call_counter += 1;
                self.close_call_triggered = true;
            }

            return;
        } else {
            self.close_call_triggered = false;
        }

        let bounds = (600.0..=1000.0, 400.0..=800.0);
        // check if the car close to intsersection مفرق
        let inside_intersection = bounds.0.contains(&self.position.0) && bounds.1.contains(&self.position.1);

        self.speed = match self.lane {
            Lane::Air => 10.5,
            _ if inside_intersection => 8.0,
            _ => 5.0,
        };
        
        if inside_intersection && self.lane != Lane::Right && self.lane != Lane::Air {
            for c in others {
                if c.id != self.id
                    && c.in_bounds(&bounds)
                    && c.id < self.id
                    && self.conflicts_with(c)
                {
                    self.speed = 0.0;
                    self.is_waiting = true;
                    return;
                }
            }
            self.is_waiting = false;
        } else if self.is_waiting {
            //check if the car can move if there not a car in the ins_area
            let mut allowed = true;
            for c in others {
                if c.id != self.id && c.in_bounds(&bounds) && c.id < self.id {
                    allowed = false;
                    break;
                }
            }
            if allowed && !self.is_car_in_front(others, 60.0) {
                for c in others {
                    if c.id != self.id &&
                        c.in_bounds(&bounds) &&
                        // && c.id < self.id  -> حاسس انه صار احسن بدونها جرب يا ذكي
                        self.conflicts_with(c)
                    {
                        self.speed = 0.0;
                        self.is_waiting = true;
                        return;
                    }
                }
                self.is_waiting = false;
            } else {
                self.speed = 0.0;
                return;
            }
        }
        // update pos of car
        if !self.is_waiting {
            if let Some(target) = self.waypoints.first() {
                let dx = target.x - self.position.0;
                let dy = target.y - self.position.1;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < self.speed {
                    self.position = (target.x, target.y);
                    if let Some(angle) = target.angle {
                        self.angle = angle;
                    }
                    self.waypoints.remove(0);
                } else {
                    let dir_x = dx / dist;
                    let dir_y = dy / dist;
                    self.position.0 += dir_x * self.speed;
                    self.position.1 += dir_y * self.speed;
                }
            }
        }
    }

    fn in_bounds(&self, bounds: &(RangeInclusive<f64>, RangeInclusive<f64>)) -> bool {
        bounds.0.contains(&self.position.0) && bounds.1.contains(&self.position.1)
    }

    fn is_car_in_front(&self, others: &[Car], safe_distance: f64) -> bool {
        for other in others {
            if other.id == self.id {
                continue;
            }

            if other.direction == self.direction && other.lane == self.lane {
                let dx = other.position.0 - self.position.0;
                let dy = other.position.1 - self.position.1;
                let dist = (dx * dx + dy * dy).sqrt();

                match self.direction {
                    Direction::North if dy > 0.0 && dist < safe_distance => {
                        return true;
                    }
                    Direction::South if dy < 0.0 && dist < safe_distance => {
                        return true;
                    }
                    Direction::East if dx < 0.0 && dist < safe_distance => {
                        return true;
                    }
                    Direction::West if dx > 0.0 && dist < safe_distance => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    fn conflicts_with(&self, other: &Car) -> bool {
        use Direction::*;
        use Lane::*;

        if self.id == other.id {
            return false;
        }

        match (self.direction, self.lane, other.direction, other.lane) {
            // Same direction
            (a, _, b, _) if a == b => false,

            // Allow North vs South if both go straight
            (North, Straight, South, Straight) => false,
            (South, Straight, North, Straight) => false,

            // Allow East vs West if both go straight
            (East, Straight, West, Straight) => false,
            (West, Straight, East, Straight) => false,

            _ => true,
        }
    }

    pub fn has_finished(&self) -> bool {
        self.waypoints.is_empty()
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let (w, h) = self.size.unwrap_or((80, 60)); // default for cars

        let rect = Rect::new(
            (self.position.0 as i32) - (w as i32) / 2,
            (self.position.1 as i32) - (h as i32) / 2,
            w,
            h,
        );

        canvas
            .copy_ex(
                self.texture,
                None,
                Some(rect),
                self.angle,
                None,
                false,
                false,
            )
            .unwrap();
    }
}
