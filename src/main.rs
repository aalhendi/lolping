use plotters::prelude::*;
use std::fs;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "2023-03-04T16-39-53/2023-03-04T16-39-53_netlog.txt";
    let contents = fs::read_to_string(file_path).expect("lole");

    let mut data = Vec::new();
    for line in contents.lines().skip(28) {
        let mut split = line.split(",");
        let cur_time = split.nth(0);
        let cur_ping = split.nth(7);
        if cur_ping.is_some() && cur_time.is_some() {
            //ping.push(cur_ping.unwrap());
            //time.push(cur_time.unwrap());
            data.push((cur_time.unwrap(), cur_ping.unwrap()));
        }
        //println!("{line}");
    }

    //println!("{ping:?}");
    //println!("{time:?}");

    let root = BitMapBackend::new("./0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("ping(ms) vs time", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(10000i32..11000i32, 0i32..200i32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter().map(|(t, p)| (t.parse::<i32>().unwrap(), p.parse::<i32>().unwrap())),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
