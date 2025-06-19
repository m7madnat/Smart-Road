    use sdl2::keyboard::Keycode;
    use sdl2::render::Texture;

    use crate::{Car, Direction, Lane, Waypoint};
    use rand::Rng;

    fn random_lane() -> Lane {
        match rand::thread_rng().gen_range(0..=2) {
            0 => Lane::Straight,
            1 => Lane::Right,
            2 => Lane::Left,
            _ => unreachable!(),
        }
    }
    pub fn spawn_car_from_key<'a>(
        key: Keycode,
        texture_pool: &'a [Texture<'a>],
        id: usize,
    ) -> Option<Car<'a>> {
        let lane = random_lane();
        let texture = &texture_pool[rand::thread_rng().gen_range(0..texture_pool.len())];

        let (direction, position, waypoints) = match key {
            Keycode::Left => {
                let direction = Direction::East;
                let (position, waypoints) = match lane {
                    Lane::Straight => (
                        (1600.0, 510.0),
                        vec![Waypoint {
                            x: -20.0,
                            y: 510.0,
                            angle: None,
                        }],
                    ),
                    Lane::Left => (
                        (1603.0, 570.0),
                        vec![
                            Waypoint {
                                x: 773.0,
                                y: 570.0,
                                angle: Some(180.0),
                            },
                            Waypoint {
                                x: 773.0,
                                y: 1240.0,
                                angle: None,
                            },
                        ],
                    ),
                    Lane::Right => (
                        (1600.0, 450.0),
                        vec![
                            Waypoint {
                                x: 950.0,
                                y: 450.0,
                                angle: Some(360.0),
                            },
                            Waypoint {
                                x: 950.0,
                                y: -40.0,
                                angle: None,
                            },
                        ],
                    ),
                    _ => return None,
                };
                (direction, position, waypoints)
            }
            Keycode::Right => {
                let direction = Direction::West;
                let (position, waypoints) = match lane {
                    Lane::Straight => (
                        (0.0, 690.0),
                        vec![Waypoint {
                            x: 1620.0,
                            y: 690.0,
                            angle: None,
                        }],
                    ),
                    Lane::Left => (
                        (0.0, 630.0),
                        vec![
                            Waypoint {
                                x: 830.0,
                                y: 630.0,
                                angle: Some(360.0),
                            },
                            Waypoint {
                                x: 830.0,
                                y: -40.0,
                                angle: None,
                            },
                        ],
                    ),
                    Lane::Right => (
                        (0.0, 750.0),
                        vec![
                            Waypoint {
                                x: 650.0,
                                y: 750.0,
                                angle: Some(180.0),
                            },
                            Waypoint {
                                x: 650.0,
                                y: 1240.0,
                                angle: None,
                            },
                        ],
                    ),
                    _ => return None,
                };
                (direction, position, waypoints)
            }
            Keycode::Up => {
                let direction = Direction::South;
                let (position, waypoints) = match lane {
                    Lane::Straight => (
                        (890.0, 1200.0),
                        vec![Waypoint {
                            x: 890.0,
                            y: -20.0,
                            angle: None,
                        }],
                    ),
                    Lane::Left => (
                        (830.0, 1200.0),
                        vec![
                            Waypoint {
                                x: 830.0,
                                y: 570.0,
                                angle: Some(270.0),
                            },
                            Waypoint {
                                x: -40.0,
                                y: 570.0,
                                angle: None,
                            },
                        ],
                    ),
                    Lane::Right => (
                        (950.0, 1200.0),
                        vec![
                            Waypoint {
                                x: 950.0,
                                y: 750.0,
                                angle: Some(90.0),
                            },
                            Waypoint {
                                x: 1640.0,
                                y: 750.0,
                                angle: None,
                            },
                        ],
                    ),
                    _ => return None,
                };
                (direction, position, waypoints)
            }
            Keycode::Down => {
                let direction = Direction::North;
                let (position, waypoints) = match lane {
                    Lane::Straight => (
                        (710.0, 0.0),
                        vec![Waypoint {
                            x: 710.0,
                            y: 1220.0,
                            angle: None,
                        }],
                    ),
                    Lane::Left => (
                        (773.0, 0.0),
                        vec![
                            Waypoint {
                                x: 773.0,
                                y: 630.0,
                                angle: Some(90.0),
                            },
                            Waypoint {
                                x: 1640.0,
                                y: 630.0,
                                angle: None,
                            },
                        ],
                    ),
                    Lane::Right => (
                        (650.0, 0.0),
                        vec![
                            Waypoint {
                                x: 650.0,
                                y: 450.0,
                                angle: Some(270.0),
                            },
                            Waypoint {
                                x: -40.0,
                                y: 450.0,
                                angle: None,
                            },
                        ],
                    ),
                    _ => return None,
                };
                (direction, position, waypoints)
            }
            _ => return None,
        };

        Some(Car::new(
            lane, position, waypoints, 5.0, id, direction, texture,None
        ))
    }
