use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::collections::HashMap;
use std::time::{Duration, Instant};

mod car;
use car::{Car, Direction, Lane, Waypoint};
use rand::Rng;
mod spawn_cars;
use spawn_cars::spawn_car_from_key;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Smart Intersection", 1600, 1200)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut cars: Vec<Car> = Vec::new();
    let mut car_id_counter = 0;
    let mut car_start_times: HashMap<usize, Instant> = HashMap::new();
    let mut car_finish_times: HashMap<usize, Instant> = HashMap::new();

    const LANE_WIDTH: u32 = 60;
    const ROAD_WIDTH: u32 = LANE_WIDTH * 6;
    const SCREEN_WIDTH: u32 = 1600;
    const SCREEN_HEIGHT: u32 = 1200;

    let center_top: u32 = 420;
    let center_bottom: u32 = 750;
    let center_left: u32 = 600;
    let center_right: u32 = 960;

    let center_x = SCREEN_WIDTH / 2;
    let center_y = SCREEN_HEIGHT / 2;

    let center_box_width = ROAD_WIDTH / 2 + 183;
    let center_box_height = ROAD_WIDTH / 2 + 183;

    let center_rect = Rect::new(
        (center_x - center_box_width / 2) as i32,
        (center_y - center_box_height / 2) as i32,
        center_box_width,
        center_box_height,
    );

    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let texture_creator = canvas.texture_creator();

    let car_textures: Vec<Texture> = vec![
        texture_creator.load_texture("assets/Car.png").unwrap(),
        texture_creator
            .load_texture("assets/Black_viper.png")
            .unwrap(),
        texture_creator.load_texture("assets/Police.png").unwrap(),
    ];

    let plane_textures: Vec<Texture> = vec![
        texture_creator.load_texture("assets/Blemheim.png").unwrap(),
        texture_creator.load_texture("assets/Hawker.png").unwrap(),
    ];

    let background_textures: Vec<Texture> = vec![
        texture_creator.load_texture("assets/left1.png").unwrap(),
        texture_creator.load_texture("assets/left2.png").unwrap(),
        texture_creator.load_texture("assets/right1.png").unwrap(),
        texture_creator.load_texture("assets/right2.png").unwrap(),
    ];

    let mut last_spawn_time = Instant::now();
    let cooldown = Duration::from_secs_f64(0.25);
    let direction_keys = [Keycode::Left, Keycode::Right, Keycode::Up, Keycode::Down];

    let mut auto_spawn_active = false;
    let mut auto_spawn_start_time = Instant::now();
    let mut close_call_count = 0;

    'running: loop {
        // Draw roads
        canvas.set_draw_color(Color::RGB(23, 23, 23));
        canvas
            .fill_rect(Rect::new(
                ((SCREEN_WIDTH - ROAD_WIDTH) / 2) as i32,
                0,
                ROAD_WIDTH,
                SCREEN_HEIGHT,
            ))
            .unwrap();
        canvas
            .fill_rect(Rect::new(
                0,
                ((SCREEN_HEIGHT - ROAD_WIDTH) / 2) as i32,
                SCREEN_WIDTH,
                ROAD_WIDTH,
            ))
            .unwrap();

        // Dashed lane dividers - - - -
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let dash_step = 60;
        for y in (0..SCREEN_HEIGHT).step_by(dash_step as usize) {
            for i in 1..=5 {
                if i == 3 {
                    continue;
                }
                let x = (SCREEN_WIDTH - ROAD_WIDTH) / 2 + i * LANE_WIDTH;
                if y < center_top || y > center_bottom {
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, 2, 30))
                        .unwrap();
                }
            }
        }
        for x in (0..SCREEN_WIDTH).step_by(dash_step as usize) {
            for i in 1..=5 {
                if i == 3 {
                    continue;
                }
                let y = (SCREEN_HEIGHT - ROAD_WIDTH) / 2 + i * LANE_WIDTH;
                if x < center_left || x > center_right {
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, 30, 2))
                        .unwrap();
                }
            }
        }
        // Solid center dividers +
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(
                ((SCREEN_WIDTH / 2) as i32) - 1,
                0,
                2,
                SCREEN_HEIGHT,
            ))
            .unwrap();
        canvas
            .fill_rect(Rect::new(
                0,
                ((SCREEN_HEIGHT / 2) as i32) - 1,
                SCREEN_WIDTH,
                2,
            ))
            .unwrap();

        // clean midd
        canvas.set_draw_color(Color::RGB(23, 23, 23));
        canvas.fill_rect(center_rect).unwrap();

        canvas.set_draw_color(Color::YELLOW);
        canvas.fill_rect(Rect::new(982, 420, 5, 179)).unwrap();
        canvas.fill_rect(Rect::new(613, 602, 5, 177)).unwrap();
        canvas.fill_rect(Rect::new(620, 414, 177, 5)).unwrap();
        canvas.fill_rect(Rect::new(802, 782, 177, 5)).unwrap();

        let image_positions = vec![(0, 0), (0, 780), (980, 0), (980, 780)];
        let (img_w, img_h) = (620, 420);
        for (i, texture) in background_textures.iter().enumerate() {
            let (x, y) = image_positions[i];
            canvas
                .copy(texture, None, Some(Rect::new(x, y, img_w, img_h)))
                .unwrap();
        }

        let a = cars.clone();
        for car in cars.iter_mut() {
            car.update_position(&a, &mut close_call_count);
            car.render(&mut canvas);
        }
        for car in &cars {
            if car.has_finished() && !car_finish_times.contains_key(&car.id) {
                car_finish_times.insert(car.id, Instant::now());
            }
        }
        cars.retain(|car| !car.has_finished());

        if auto_spawn_active {
            if auto_spawn_start_time.elapsed() < Duration::from_secs(60) {
                if last_spawn_time.elapsed() >= cooldown + cooldown {
                    let random_key =
                        direction_keys[rand::thread_rng().gen_range(0..direction_keys.len())];
                    if let Some(car) = spawn_car_from_key(random_key, &car_textures, car_id_counter)
                    {
                        car_start_times.insert(car_id_counter, Instant::now());
                        cars.push(car);
                        car_id_counter += 1;
                        last_spawn_time = Instant::now();
                    }
                }
            } else {
                auto_spawn_active = false;
            }
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(key), ..
            } = event
            {
                match key {
                    Keycode::Escape => {
                        break 'running;
                    }
                    Keycode::R => {
                        auto_spawn_active = true;
                        auto_spawn_start_time = Instant::now();
                    }
                    Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                        if last_spawn_time.elapsed() >= cooldown {
                            if let Some(car) =
                                spawn_car_from_key(key, &car_textures, car_id_counter)
                            {
                                car_start_times.insert(car_id_counter, Instant::now());
                                cars.push(car);
                                car_id_counter += 1;
                                last_spawn_time = Instant::now();
                            }
                        }
                    }
                    Keycode::P => {
                        let texture =
                            &plane_textures[rand::thread_rng().gen_range(0..plane_textures.len())];

                        let lane = Lane::Air;
                        let direction = Direction::East;
                        let position = (1620.0, 1000.0);

                        let waypoints = vec![Waypoint {
                            x: -20.0,
                            y: 170.0,
                            angle: None,
                        }];

                        cars.push(Car::new(
                            lane,
                            position,
                            waypoints,
                            4.0,
                            car_id_counter,
                            direction,
                            texture,
                            Some((120, 80)),
                        ));
                        car_id_counter += 1;
                    }
                    _ => {}
                }
            } else if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }

    // === Show stats window after ESC ===
    let ttf_context = sdl2::ttf::init().expect("Failed to init TTF");
    let font = ttf_context.load_font("assets/Roboto.ttf", 32).unwrap();

    let stats_window = video_subsystem
        .window("Simulation Stats", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut stats_canvas = stats_window.into_canvas().build().unwrap();
    let texture_creator = stats_canvas.texture_creator();

    let mut max_duration = Duration::ZERO;
    let mut min_duration: Option<Duration> = None;
    for (id, finish_time) in &car_finish_times {
        if let Some(start_time) = car_start_times.get(id) {
            let duration = *finish_time - *start_time;
            if duration > max_duration {
                max_duration = duration;
            }
            match min_duration {
                Some(min) if duration < min => {
                    min_duration = Some(duration);
                }
                None => {
                    min_duration = Some(duration);
                }
                _ => {}
            }
        }
    }

    let text_surface_1 = font
        .render(&format!("Total Cars Spawned: {}", car_id_counter))
        .blended(Color::WHITE)
        .unwrap();
    let text_texture_1 = texture_creator
        .create_texture_from_surface(&text_surface_1)
        .unwrap();

    let text_surface_2 = font
        .render(&format!("Max Time: {:.2?}", max_duration))
        .blended(Color::WHITE)
        .unwrap();
    let text_texture_2 = texture_creator
        .create_texture_from_surface(&text_surface_2)
        .unwrap();

    let text_surface_3 = font
        .render(&format!("Min Time: {:.2?}", min_duration.unwrap()))
        .blended(Color::WHITE)
        .unwrap();
    let text_texture_3 = texture_creator
        .create_texture_from_surface(&text_surface_3)
        .unwrap();

    let text_surface_4 = font
        .render(&format!("Max Speed: {:.2} px/frame", 8))
        .blended(Color::WHITE)
        .unwrap();
    let text_texture_4 = texture_creator
        .create_texture_from_surface(&text_surface_4)
        .unwrap();

    let text_surface_5 = font
        .render(&format!("Min Speed: {:.2} px/frame", 5))
        .blended(Color::WHITE)
        .unwrap();
    let text_texture_5 = texture_creator
        .create_texture_from_surface(&text_surface_5)
        .unwrap();

    let text_surface_6 = font
        .render(&format!("Close Calls: {}", close_call_count))
        .blended(Color::WHITE)
        .unwrap();
    let text_texture_6 = texture_creator
        .create_texture_from_surface(&text_surface_6)
        .unwrap();
    'stats_loop: loop {
        stats_canvas.set_draw_color(Color::RGB(0, 0, 0));
        stats_canvas.clear();
        stats_canvas
            .copy(&text_texture_1, None, Some(Rect::new(50, 60, 400, 40)))
            .unwrap();
        stats_canvas
            .copy(&text_texture_2, None, Some(Rect::new(50, 110, 400, 40)))
            .unwrap();
        stats_canvas
            .copy(&text_texture_3, None, Some(Rect::new(50, 160, 400, 40)))
            .unwrap();
        stats_canvas
            .copy(&text_texture_4, None, Some(Rect::new(50, 210, 400, 40)))
            .unwrap();
        stats_canvas
            .copy(&text_texture_5, None, Some(Rect::new(50, 260, 400, 40)))
            .unwrap();
        stats_canvas
            .copy(&text_texture_6, None, Some(Rect::new(50, 310, 400, 40)))
            .unwrap();
        stats_canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'stats_loop;
                }
                _ => {}
            }
        }
    }
    std::thread::sleep(Duration::from_secs(1));
}
