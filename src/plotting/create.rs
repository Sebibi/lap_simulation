use std::error::Error;
use crate::models::base_model::Model;
use crate::tracks::base_track::Track;
use plotters::prelude::*;

/// Plot both the track and the model to a single SVG file
/// 
/// # Arguments
/// * `track_obj` - Reference to the track to plot
/// * `model_obj` - Reference to the model to plot
/// * `filename` - Path to save the combined plot (e.g., "output.svg")
/// 
/// # Returns
/// Result indicating success or error
pub fn plot<M: Model + ?Sized>(
    track_obj: &dyn Track,
    model_obj: &M,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new(filename, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let (min_coord, max_coord) = track_obj.get_plot_range();
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Track and Model", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_coord..max_coord, min_coord..max_coord)?;
    
    chart.configure_mesh().draw()?;
    
    // Plot track outside boundary
    chart.draw_series(LineSeries::new(
        track_obj.get_outside_boundary().iter().map(|&(x, y)| (x, y))
            .chain(std::iter::once(track_obj.get_outside_boundary()[0])),
        &BLACK,
    ))?
    .label("Outside Boundary")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    
    // Plot track center line (dotted)
    chart.draw_series(
        track_obj.get_center_line().iter().map(|&(x, y)| (x, y))
            .chain(std::iter::once(track_obj.get_center_line()[0]))
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
    
    // Plot track inside boundary
    chart.draw_series(LineSeries::new(
        track_obj.get_inside_boundary().iter().map(|&(x, y)| (x, y))
            .chain(std::iter::once(track_obj.get_inside_boundary()[0])),
        &BLACK,
    ))?
    .label("Inside Boundary")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    
    // Plot track start position
    let start_pos = track_obj.get_start_position();
    chart.draw_series(std::iter::once(Circle::new(
        (start_pos.0, start_pos.1),
        5,
        BLACK.filled(),
    )))?
    .label("Start Position")
    .legend(|(x, y)| Circle::new((x + 10, y), 5, BLACK.filled()));
    
    // Plot model
    let (x, y, yaw) = model_obj.get_position();
    let (length, width) = model_obj.get_size();
    
    // Calculate the four corners of the rectangle in body frame
    let half_length = length / 2.0;
    let half_width = width / 2.0;
    
    let corners_body = [
        (half_length, half_width),
        (-half_length, half_width),
        (-half_length, -half_width),
        (half_length, -half_width),
    ];
    
    // Transform corners to world frame using yaw rotation
    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();
    
    let corners_world: Vec<(f64, f64)> = corners_body
        .iter()
        .map(|(x_body, y_body)| {
            let x_world = x + x_body * cos_yaw - y_body * sin_yaw;
            let y_world = y + x_body * sin_yaw + y_body * cos_yaw;
            (x_world, y_world)
        })
        .collect();
    
    // Draw filled rectangle for model
    chart.draw_series(std::iter::once(Polygon::new(
        corners_world.clone(),
        &BLUE.mix(0.5),
    )))?
    .label("Vehicle")
    .legend(|(x, y)| Rectangle::new([(x, y), (x + 20, y + 10)], BLUE.mix(0.5).filled()));
    
    // Draw rectangle outline
    let mut outline = corners_world.clone();
    outline.push(corners_world[0]); // Close the polygon
    chart.draw_series(LineSeries::new(
        outline,
        ShapeStyle::from(&BLUE).stroke_width(2),
    ))?;
    
    // Draw orientation arrow (pointing in the direction of positive x in body frame)
    let arrow_length = length * 0.6;
    let arrow_x = x + arrow_length * cos_yaw;
    let arrow_y = y + arrow_length * sin_yaw;
    
    chart.draw_series(LineSeries::new(
        vec![(x, y), (arrow_x, arrow_y)],
        ShapeStyle::from(&GREEN).stroke_width(3),
    ))?;
    
    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    
    root.present()?;
    println!("Combined plot saved to {}", filename);
    Ok(())
}
