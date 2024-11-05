use crate::data_point::DataPoint;
use plotters::prelude::*;
use std::error::Error;

pub fn plot_data(data_points: &Vec<DataPoint>) -> Result<(), Box<dyn Error>> {
    // Determine min and max for axes
    let min_time = 0.0;
    let max_time = data_points.iter().map(|d| d.time).fold(0.0, f64::max);

    let min_hp = data_points
        .iter()
        .map(|d| d.hp)
        .fold(f64::INFINITY, f64::min);
    let max_hp = data_points
        .iter()
        .map(|d| d.hp)
        .fold(f64::NEG_INFINITY, f64::max);

    let min_hx = data_points
        .iter()
        .map(|d| d.hx)
        .fold(f64::INFINITY, f64::min);
    let max_hx = data_points
        .iter()
        .map(|d| d.hx)
        .fold(f64::NEG_INFINITY, f64::max);

    let min_torb = data_points
        .iter()
        .map(|d| d.torb)
        .fold(f64::INFINITY, f64::min);
    let max_torb = data_points
        .iter()
        .map(|d| d.torb)
        .fold(f64::NEG_INFINITY, f64::max);

    // Plot HP vs Time
    let root = BitMapBackend::new("hp_vs_time.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("HP vs Time", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_time..max_time, min_hp..max_hp)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(
        data_points
            .iter()
            .map(|point| Circle::new((point.time, point.hp), 5, RED.filled())),
    )?;
    root.present()?;

    // Plot HX vs Time
    let root = BitMapBackend::new("hx_vs_time.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("HX vs Time", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_time..max_time, min_hx..max_hx)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(
        data_points
            .iter()
            .map(|point| Circle::new((point.time, point.hx), 5, BLUE.filled())),
    )?;
    root.present()?;

    // Plot Torb vs Time
    let root = BitMapBackend::new("torb_vs_time.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Torb vs Time", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_time..max_time, min_torb..max_torb)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(
        data_points
            .iter()
            .map(|point| Circle::new((point.time, point.torb), 5, GREEN.filled())),
    )?;
    root.present()?;

    Ok(())
}
