use plotters::prelude::*;
use std::{error::Error, fs};

// struct Entry {
//     time: i32,
//     address: String,
//     incoming: i32,
//     outgoing: i32,
//     app_ctos: i32,
//     app_stoc: i32,
//     loss: i32,
//     sent: i32,
//     ping: i32,
//     variance: i32,
//     reliable_delayed: i32,
//     unreliable_delayed: i32,
//     app_update_delayed: i32,
//     time_spent_critical_frame: i32,
//     latency_window: i32,
//     packet_loss_percent_window: f32,
//     jitter: i32,
//     overall_latency_min: i32,
//     overall_latency_max: i32,
//     latency_min: i32,
//     latency_max: i32,
//     latency_packet_samples: i32,
//     jitter_packet_samples: i32,
//     reconnect: i32,
// }

// TODO: User selects features (command line args?)
const IS_GRID_ENABLED: bool = false;
const IS_POINTS_ENABLED: bool = true;
const IS_LINE_ENABLED: bool = true;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "2023-03-04T16-39-53/2023-03-04T16-39-53_netlog.txt";
    let contents = fs::read_to_string(file_path).expect("Unable to read file contents");

    let mut data = Vec::new();
    let mut max_ping = i32::MIN; // Initialize maximum to the minimum value of i32

    for line in contents.lines().skip(28) {
        let mut split = line.split(',');
        if let (Some(cur_time), Some(cur_ping)) = (split.next(), split.nth(7)) {
            let cur_time =
                cur_time.parse::<f64>().expect("Could not parse timestamp") / 1000.0 / 60.0;
            let cur_ping = cur_ping.parse::<i32>().expect("Could not parse ping");

            // Keep track of maximum
            if cur_ping > max_ping {
                max_ping = cur_ping;
            }

            data.push((cur_time, cur_ping));
        }
    }

    let root = BitMapBackend::new("./0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = data.last().expect("Could not obtain max_time").0 * 1.05;
    let next_fifty = ((max_ping as f64 / 50.0).ceil() * 50.0) as i32;

    let mut chart = ChartBuilder::on(&root)
        .caption("Ping vs Time", ("sans-serif", 32).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(55)
        .build_cartesian_2d(0f64..max_time, 0i32..next_fifty)?;

    // TODO: Clean up grid conditional implementation
    if IS_GRID_ENABLED {
        chart
            .configure_mesh()
            .x_desc("Time (minutes)")
            .y_desc("Ping (ms)")
            .axis_desc_style(("sans-serif", 15))
            .x_label_style(("sans-serif", 15))
            .y_label_style(("sans-serif", 15));
    } else {
        chart
            .configure_mesh()
            .x_desc("Time (minutes)")
            .y_desc("Ping (ms)")
            .axis_desc_style(("sans-serif", 15))
            .x_label_style(("sans-serif", 15))
            .y_label_style(("sans-serif", 15))
            .disable_mesh()
            .draw()?;
    }

    if IS_LINE_ENABLED {
        chart
            .draw_series(LineSeries::new(data.iter().map(|(t, p)| (*t, *p)), &RED))?
            .label("Ping")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    }

    if IS_POINTS_ENABLED {
        chart.draw_series(
            data.iter()
                .map(|(t, p)| Circle::new((*t, *p), 2, RED.filled())),
        )?;
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
