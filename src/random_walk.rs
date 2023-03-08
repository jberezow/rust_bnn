// Random Walk Metropolis Sampler

// use na::Vector2;
use crate::blr::Model;
use crate::point2::Point2;
// use crate::dataset::Cloud;
use rand_distr::{Normal, Distribution};

pub fn gaussian_perturbation(model: &mut Model) {
    let w0_dist = Normal::new(0.0, 0.3).unwrap();
    let w1_dist = Normal::new(0.0, 0.3).unwrap();
    let b_dist = Normal::new(0.0, 0.3).unwrap();
    model.w[0] += w0_dist.sample(&mut rand::thread_rng());
    model.w[1] += w1_dist.sample(&mut rand::thread_rng());
    model.b += b_dist.sample(&mut rand::thread_rng());
}

pub fn single_step_metropolis<'a>(model: &'a mut Model, points: &'a Vec<Point2>) -> &'a mut Model {

    // Generate a new model
    let mut new_model: Model = Model::default();

    // Copy model params from old model
    new_model.w = model.w.clone();
    new_model.b = model.b;

    // Add random normal perturbation to model params
    gaussian_perturbation(&mut new_model);

    // Calculate the log likelihood of the new model
    let new_log_likelihood = new_model.log_likelihood(points);

    // Calculate the log likelihood of the old model
    let old_log_likelihood = model.log_likelihood(points);

    // Calculate the log prior of the new model
    let new_log_prior = new_model.log_prior();

    // Calculate the log prior of the old model
    let old_log_prior = model.log_prior();

    // Calculate the log posterior of the new model
    let new_log_posterior = new_log_likelihood + new_log_prior;

    // Calculate the log posterior of the old model
    let old_log_posterior = old_log_likelihood + old_log_prior;

    // Calculate the log acceptance probability
    let log_acceptance_probability = new_log_posterior - old_log_posterior;

    // Accept the new model if the log acceptance probability is greater than 0
    if log_acceptance_probability > 0.0 {
        *model = new_model;
        println!("Accepted superior model with log acceptance probability: {}", log_acceptance_probability);
    }
    else if log_acceptance_probability < 0.0 {
        // Generate a random number between 0 and 1
        let random_number = rand::random::<f64>();

        // Accept the new model if the random number is less than the acceptance probability
        if random_number < (log_acceptance_probability).exp() {
            *model = new_model;
            println!("Accepted inferior model with log acceptance probability: {} and random number {}", log_acceptance_probability.exp(), random_number);
        }
        else {
            println!("Sticking with old model with log acceptance probability {}", log_acceptance_probability);
        }
    } 

    model
}

pub fn random_walk_metropolis<'a>(model: &'a mut Model, points: &'a Vec<Point2>, n: usize) -> Vec<Model>{
    // Init the samples
    let mut samples: Vec<Model> = Vec::new();

    // Clone the model for first step
    let mut sample: Model = model.clone();

    // Run a single step metropolis sampler for n iterations
    for _ in 0..n {
        let sample: &mut Model = single_step_metropolis(&mut sample, points);
        let record_state: Model = sample.clone();
        samples.push(record_state);
    }

    samples
}