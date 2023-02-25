use plotters::prelude::*;
use crate::cloud::Cloud;
use std::fs::File;
use std::io::Write;

// 2D Cartesian Plot of Clouds
pub struct CloudPlot {
  pub clouds: Vec<Cloud>,
}

// Implement the CloudPlot struct
impl CloudPlot{
  pub fn new(clouds: Vec<Cloud>) -> CloudPlot {
    CloudPlot { clouds }
  }

  // Render the plot, with each cloud coloured according to class
  pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Delete existing plot and file if they exist
    std::fs::remove_file("plotters-doc-data/Plot.png").unwrap_or(());
    std::fs::remove_file("Data.txt").unwrap_or(());

    // Drawing area and file for points
    let root = BitMapBackend::new("plotters-doc-data/Plot.png", (640, 480)).into_drawing_area();
    let mut file = File::create("Data.txt").unwrap();

    // Fill the background with white
    root.fill(&WHITE)?;

    // Create a 2d cartesian coordinate system
    let mut chart = ChartBuilder::on(&root)
      .caption("XOR Cloud Data", ("sans-serif", 30).into_font())
      .margin(5)
      .x_label_area_size(30)
      .y_label_area_size(30)
      .build_cartesian_2d(-1.0..1.0, -1.0..1.0)?;

    // Configure the coordinate system
    chart.configure_mesh().draw()?;

    // Plot each cloud
    for cloud in &self.clouds {
      // Get the points from the cloud
      let points = cloud.points.as_ref().unwrap();

      // Get the colour for the cloud
      let colour = match cloud.y {
        0 => RED,
        1 => BLUE,
        _ => BLACK,
      };

      // Add points to the file without overwriting it
      for p in points {
        writeln!(file, "{} {}", p.x[0], p.x[1])?;
      }

      // Plot the points
      chart.draw_series(
        points.iter().map(|p| Circle::new((p.x[0], p.x[1]), 4.0, colour.filled()))
      )?;
    }

    Ok(())
  }
}
