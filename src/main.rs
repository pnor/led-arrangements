// use led_arrangements::LightStrip;
use led_arrangements::{builder, ArrangementConfig, Color, LightArrangement, Loc, TestStrip};
use std::{thread::sleep, time::Duration};

fn main() {
    let light_loc = vec![
        ([0.1, 0.1, 0.1, 0.2], 0),
        ([0.9, 0.9, 0.9, 0.2], 1),
        ([0.5, 0.5, 0.5, 0.2], 2),
        ([0.1, 0.9, 0.1, 0.2], 3),
        ([0.9, 0.1, 0.9, 0.2], 4),
        ([0.1, 0.1, 0.9, 0.2], 5),
        ([0.1, 0.9, 0.9, 0.2], 6),
        ([0.9, 0.9, 0.1, 0.2], 7),
    ];

    let arrangement_config = ArrangementConfig {
        light_locations: light_loc,
    };

    let strip = builder::test(&arrangement_config, &[0, 1, 2]);

    let mut light_arrangement: LightArrangement<TestStrip, 4> =
        LightArrangement::new(strip, arrangement_config);

    light_arrangement.fill(&Color {
        red: 255,
        green: 10,
        blue: 20,
    });
    light_arrangement.show();

    let mut red_comp = 255;
    loop {
        light_arrangement.set_decreasing_intensity(
            &Loc::cartesian([0.5, 0.5, 0.5, 0.5]),
            &Color {
                red: red_comp,
                green: 255,
                blue: red_comp,
            },
            0.95,
        );
        if red_comp == 255 {
            red_comp = 20;
        } else {
            red_comp = 255;
        }

        light_arrangement.show();

        std::thread::sleep(Duration::from_millis(50));
    }
}
