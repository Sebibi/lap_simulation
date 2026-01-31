use std::error::Error;
use crate::models::base_model::Model;
use crate::tracks::base_track::Track;
use super::track;
use super::model;

/// Plot both the track and the model to separate SVG files
/// 
/// # Arguments
/// * `track_obj` - Reference to the track to plot
/// * `model_obj` - Reference to the model to plot
/// * `filename` - Base path to save the plots (e.g., "output.svg" -> "output_track.svg" and "output_model.svg")
/// 
/// # Returns
/// Result indicating success or error
pub fn plot<M: Model + ?Sized>(
    track_obj: &dyn Track,
    model_obj: &M,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // For now, we'll create separate plots using the individual plotting functions
    // You can extend this to create a combined visualization if needed
    
    let track_filename = filename.replace(".svg", "_track.svg");
    let model_filename = filename.replace(".svg", "_model.svg");
    
    track::plot_track(track_obj, &track_filename)?;
    model::plot_model(model_obj, &model_filename)?;
    
    println!("Combined plots saved to {} and {}", track_filename, model_filename);
    Ok(())
}
