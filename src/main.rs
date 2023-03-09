mod point2;
mod dataset;
mod blr;
mod prior;
mod basic_plot;
mod model_plot;
mod random_walk;
mod activation;

pub use na::Vector2;
pub use point2::Point2;
pub use basic_plot::CloudPlot;
pub use model_plot::ClassifierPlot;
pub use blr::Model;
pub use random_walk::random_walk_metropolis;

fn main() {
    // Variables
    let c = 4;
    let n = 500;
    let std: Vector2<f64> = Vector2::new(0.2, 0.2);
    let model_samples = 10000;

    // Generate four clouds in each quadrant of the cartesian plane
    let mut clouds = Vec::new();
    for i in 0..c {
        let x0 = f64::from(i % 2) - 0.5;
        let x1 = f64::from(i / 2) - 0.5;
        let y = i8::from(!(i == 0 || i == c - 1));
        let cloud = dataset::Cloud::new(Vector2::new(x0, x1), std, n, y);
        clouds.push(cloud);
    }

    let mut points: Vec<Point2> = Vec::new();
    for cloud in &clouds {
        points.extend(cloud.points.clone().unwrap());
    }

    // // Init the model samples
    // let mut models = Vec::new();
    // for _ in 0..model_samples {
    //     // Default model
    //     let mut model: Model = Model::default();

    //     // Draw from the prior
    //     model.draw_from_prior();

    //     models.push(model);
    // }

    // Variables
    // let n = 1000;
    // let noise = 0.7;
    // let model_samples = 10000;

    // let moons = dataset::Moons::new(noise, n);

    // let points = moons.points.clone().unwrap();

    // Plot classification after random walk metropolis
    let rw_title: String = "Random Walk Metropolis".to_string();
    let mut model: Model = Model::default();

    // Sample random walk four times
    let l = 1;
    let mut rw_models = random_walk_metropolis(&mut model, &points, model_samples/l);
    for _ in 0..l-1 {
        rw_models.extend(random_walk_metropolis(&mut model, &points, model_samples/l));
    }

    // Use every 100th model
    let rw_models = rw_models.iter().step_by(400).cloned().collect::<Vec<Model>>();
    
    let class_plot = ClassifierPlot::new(rw_models, points, rw_title);
    class_plot.render().unwrap();

}
