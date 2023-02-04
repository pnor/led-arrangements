// use led_arrangements::LightStrip;
use led_arrangements::{strip_builder, ArrangementConfig, Color, LightArrangement, Loc, TestStrip};
use std::{f64::consts::PI, time::Duration};

fn get_light_locs() -> Vec<([f64; 3], usize)> {
    let mut counter = 0;
    let mut vec = vec![];
    for i in 0..=10 {
        for j in 0..=10 {
            for k in 0..=10 {
                vec.push((
                    [(i as f64 / 10.0), (j as f64 / 10.0), (k as f64 / 10.0)],
                    counter,
                ));
                counter += 1;
            }
        }
    }
    return vec;
}

fn main() {
    let arrangement_config = ArrangementConfig {
        light_locations: get_light_locs(),
    };

    let strip = strip_builder::test(&arrangement_config, &[0, 1, 2]);

    let mut light_arrangement: LightArrangement<TestStrip, 3> =
        LightArrangement::new(strip, arrangement_config);

    light_arrangement.fill(&Color {
        red: 0,
        green: 0,
        blue: 0,
    });
    light_arrangement.show();

    let color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    let mut prog = 0.0;
    loop {
        light_arrangement.fill(&Color {
            red: 0,
            green: 0,
            blue: 0,
        });
        light_arrangement.set_decreasing_intensity(
            &Loc::polar(0.4, &vec![PI / 2.0, (PI * 2.0 * prog)], &[0.5, 0.5, 0.5]),
            0.3,
            &color,
        );

        light_arrangement.show();

        prog = (prog + 0.03) % 1.0;

        std::thread::sleep(Duration::from_millis(25));
    }
}
