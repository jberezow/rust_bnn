use plotters::prelude::*;
use std::fs::File;
use std::io::Write;
use na::Vector2;

use crate::cloud::Cloud;
use crate::blr::Model;
use crate::point2::Point2;

// 2D plot of classifier and data
pub struct ClassifierPlot {
  pub models: Vec<Model>,
  pub clouds: Vec<Cloud>,
  pub title: String,
}

// Implement the ClassifierPlot struct
impl ClassifierPlot {
  pub fn new(models: Vec<Model>, clouds: Vec<Cloud>, title: String) -> ClassifierPlot {
    ClassifierPlot {
      models,
      clouds,
      title,
    }
  }

  // Render the plot, with each cloud coloured according to class
  pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Print out Plot Title
    println!("{}","=".repeat(50));
    println!("{}{}{}",
      "=*=".repeat(5),
      self.title,
      "*=*".repeat(5)
    );
    println!("{}","=".repeat(50));

    // Delete existing plot and file if they exist
    let path: String = format!("plotters-doc-data/{}.png", self.title);
    std::fs::remove_file(&path).unwrap_or(());
    std::fs::remove_file("Data.txt").unwrap_or(());

    // Drawing area and file for points
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
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


    for (i, model) in self.models.iter().enumerate() {
      println!("Plotting model number {}", i);
      // Plot the classifier
      let mut x: Vector2<f64> = Vector2::new(-1.0, -1.0);
      while x[0] <= 1.0 {
        while x[1] <= 1.0 {
          let y = model.forward(x);
          let point: Point2 = Point2::new(x, y);
          
          // Plot the point according to its colour with 0.1 alpha
          let colour = match y {
            0 => RED.mix(0.01),
            1 => BLUE.mix(0.01),
            _ => BLACK.mix(0.01),
          };

          // Plot only this one point
          let mut plot_now: Vec<Point2> = Vec::new();
          plot_now.push(point);
          chart.draw_series(
            plot_now.iter().map(|p| Circle::new((p.x[0], p.x[1]), 4.0, colour.filled()))
          )?;

          x[1] += 0.02;
        }

        x[0] += 0.02;
        x[1] = -1.0;
      }
    }

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
      