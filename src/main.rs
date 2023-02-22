use light_arrangements::{
    ArrangementConfig, Color, LightArrangement, LightStripConfig, Loc, TestStrip,
    TestStripDisplayConfig,
};
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
        number_children_for_division: 200,
    };

    let display_config = TestStripDisplayConfig::default();

    let strip = TestStrip::new(&arrangement_config, &display_config);

    let mut light_arrangement: LightArrangement<TestStrip, 3> =
        LightArrangement::new(strip, arrangement_config).unwrap();

    light_arrangement.fill(&Color {
        red: 0,
        green: 0,
        blue: 0,
    });
    light_arrangement.show();

    let color1 = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    let color2 = Color {
        red: 0,
        green: 0,
        blue: 255,
    };

    let mut prog = 0.0;
    let mut count = 0;
    loop {
        if {
            let tmp = count;
            count += 1;
            tmp
        } > 10000
        {
            break;
        }

        light_arrangement.fill(&Color {
            red: 0,
            green: 0,
            blue: 0,
        });
        light_arrangement.set_decreasing_intensity_merge(
            &Loc::polar(0.4, &vec![PI / 2.0, (PI * 2.0 * prog)], &[0.5, 0.5, 0.5]),
            0.3,
            &color1,
        );
        light_arrangement.set_decreasing_intensity_merge(
            &Loc::polar(0.4, &vec![(PI * 2.0 * prog), PI / 2.0], &[0.5, 0.5, 0.5]),
            0.3,
            &color2,
        );

        light_arrangement.show();

        prog = (prog + 0.03) % 1.0;

        std::thread::sleep(Duration::from_millis(25));
    }
}
