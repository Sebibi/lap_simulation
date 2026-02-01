use plotters::prelude::*;
use std::error::Error;
use crate::models::base_model::Model;

/// Plot a model as a rectangle to an SVG file
/// 
/// # Arguments
/// * `model` - Reference to the model to plot
/// * `path` - File path for the output SVG
/// 
/// # Returns
/// Result indicating success or error
pub fn plot_model<M: Model + ?Sized>(model: &M, path: &str) -> Result<(), Box<dyn Error>> {
    let (x, y, yaw) = model.get_position();
    let (length, width) = model.get_size();
    
    // Create plot area with padding around the model
    let padding = length.max(width) * 2.0;
    let x_min = x - padding;
    let x_max = x + padding;
    let y_min = y - padding;
    let y_max = y + padding;
    
    let root = SVGBackend::new(path, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Model Position", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    
    chart.configure_mesh()
        .x_desc("X (m)")
        .y_desc("Y (m)")
        .draw()?;
    
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
    
    // Draw filled rectangle
    chart.draw_series(std::iter::once(Polygon::new(
        corners_world.clone(),
        &RGBColor(150, 150, 150).mix(0.7),
    )))?;
    
    // Draw rectangle outline
    let mut outline = corners_world.clone();
    outline.push(corners_world[0]); // Close the polygon
    chart.draw_series(LineSeries::new(
        outline,
        &BLACK,
    ))?;
    
    // Draw orientation arrow (pointing in the direction of positive x in body frame)
    let arrow_length = length * 0.6;
    let arrow_x = x + arrow_length * cos_yaw;
    let arrow_y = y + arrow_length * sin_yaw;
    
    chart.draw_series(LineSeries::new(
        vec![(x, y), (arrow_x, arrow_y)],
        ShapeStyle::from(&RED).stroke_width(3),
    ))?;
    
    root.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::plot_model;
    use crate::models::point_mass::PointMass;

    #[test]
    fn test_point_mass_plot_model() {
        use std::f64::consts::PI;

        // Create a model at position (10, 20) with yaw = PI/4 (45 degrees)
        let mut model = PointMass::with_initial_state(10.0, 20.0, 0.0, PI / 4.0);
        model.set_size(5.0, 2.0);

        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let filename = temp_dir.path().join("test_model_plot.svg");

        // Plot the model
        let result = plot_model(&model, filename.to_str().expect("temp path not utf-8"));
        assert!(result.is_ok(), "Failed to plot model: {:?}", result.err());

        // Verify file was created
        assert!(filename.exists(), "Plot file was not created");
    }
}
