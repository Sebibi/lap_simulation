use plotters::prelude::*;
use std::error::Error;
use crate::tracks::base_track::Track;

/// Plot a track to an SVG file
/// 
/// # Arguments
/// * `track` - Reference to the track to plot
/// * `filename` - Path to save the plot (e.g., "track.svg")
/// 
/// # Returns
/// Result indicating success or error
pub fn plot_track(track: &dyn Track, filename: &str) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new(filename, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let (min_coord, max_coord) = track.get_plot_range();
    
    let mut chart = ChartBuilder::on(&root)
        .caption(track.get_track_name(), ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_coord..max_coord, min_coord..max_coord)?;
    
    chart.configure_mesh().draw()?;
    
    // Plot outside boundary
    chart.draw_series(LineSeries::new(
        track.get_outside_boundary().iter().map(|&(x, y)| (x, y))
            .chain(std::iter::once(track.get_outside_boundary()[0])),
        &BLACK,
    ))?
    .label("Outside Boundary")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    
    // Plot center line (dotted)
    chart.draw_series(
        track.get_center_line().iter().map(|&(x, y)| (x, y))
            .chain(std::iter::once(track.get_center_line()[0]))
            .collect::<Vec<_>>()
            .windows(2)
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .flat_map(|(_, w)| {
                vec![
                    PathElement::new(vec![w[0], w[1]], RED.stroke_width(2))
                ]
            })
    )?
    .label("Center Line")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));
    
    // Plot inside boundary
    chart.draw_series(LineSeries::new(
        track.get_inside_boundary().iter().map(|&(x, y)| (x, y))
            .chain(std::iter::once(track.get_inside_boundary()[0])),
        &BLACK,
    ))?
    .label("Inside Boundary")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    
    // Plot start position
    let start_pos = track.get_start_position();
    chart.draw_series(std::iter::once(Circle::new(
        (start_pos.0, start_pos.1),
        5,
        BLACK.filled(),
    )))?
    .label("Start Position")
    .legend(|(x, y)| Circle::new((x + 10, y), 5, BLACK.filled()));
    
    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    
    root.present()?;
    println!("{} plot saved to {}", track.get_track_name(), filename);
    Ok(())
}
