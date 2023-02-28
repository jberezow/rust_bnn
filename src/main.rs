mod point2;
mod cloud;
mod blr;
mod basic_plot;
mod model_plot;

pub use na::Vector2;
pub use point2::Point2;
pub use basic_plot::CloudPlot;
pub use model_plot::ClassifierPlot;
pub use blr::Model;

fn main() {
    // Variables
    let c = 4;
    let n = 100;
    let std: Vector2<f64> = Vector2::new(0.1, 0.1);
    let model_samples = 50;

    // Generate four clouds in each quadrant of the cartesian plane
    let mut clouds = Vec::new();
    for i in 0..c {
        let x0 = (i % 2) as f64 - 0.5;
        let x1 = (i / 2) as f64 - 0.5;
        let y = if i == 0 || i == c - 1 { 0 } else { 1 };
        let cloud = cloud::Cloud::new(Vector2::new(x0, x1), std, n, y as i8);
        clouds.push(cloud);
    }

    // Init the cloud plot
    // let plot = CloudPlot::new(clouds);

    // Render the plot and save it to a file
    // plot.render().unwrap();

    // Init the model samples
    let mut models = Vec::new();
    for _ in 0..model_samples {
        let mut model = Model::new();

        // Draw from the prior
        model.draw_from_prior();

        models.push(model);
    }

    // Plot naive classification
    let naive_title: String = "Naive Classification".to_string();
    let class_plot = ClassifierPlot::new(models, clouds, naive_title);
    class_plot.render().unwrap();
}
