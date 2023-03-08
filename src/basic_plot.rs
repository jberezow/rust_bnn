use plotters::prelude::*;
use std::fs::File;
use std::io::Write;

// use crate::dataset::Cloud;
use crate::point2::Point2;

// 2D Cartesian Plot of Clouds
pub struct CloudPlot {
  pub data: Vec<Point2>,
  
}

// Implement the CloudPlot struct
impl CloudPlot{
  #[must_use] pub fn new(data: Vec<Point2>) -> Self {
    Self { data }
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

    // Plot the axes
    chart.draw_series(LineSeries::new(
      vec![(0.0, -1.0), (0.0, 1.0)],
      &BLACK,
    ))?;
    chart.draw_series(LineSeries::new(
      vec![(-1.0, 0.0), (1.0, 0.0)],
      &BLACK,
    ))?;

    // Plot the data
    for point in &self.data {

      // Get the colour for the point
      let colour = match point.y {
        0 => RED,
        1 => BLUE,
        _ => BLACK,
      };

      writeln!(file, "{} {}", point.x[0], point.x[1])?;

      // Plot the point according to colour
      chart.draw_series(
        std::iter::once(Circle::new((point.x[0], point.x[1]), 4.0, colour.filled()))
      )?;
    }

    Ok(())
  }
}