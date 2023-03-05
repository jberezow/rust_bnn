// Random Walk Metropolis Sampler

// use na::Vector2;
use crate::blr::Model;
// use crate::point2::Point2;
use crate::cloud::Cloud;

pub fn single_step_metropolis<'a>(model: &'a mut Model, clouds: &'a Vec<Cloud>) -> &'a mut Model {

    // Generate a new model
    let mut new_model: Model = Model::default();

    // Draw from the prior
    new_model.draw_from_prior();

    // Calculate the log likelihood of the new model
    let new_log_likelihood = new_model.log_likelihood(clouds);

    // Calculate the log likelihood of the old model
    let old_log_likelihood = model.log_likelihood(clouds);

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
        println!("Accepted new model with log acceptance probability: {}", log_acceptance_probability);
    }
    else if log_acceptance_probability < 0.0 {
        // Generate a random number between 0 and 1
        let random_number = rand::random::<f64>();

        // Accept the new model if the random number is less than the acceptance probability
        if random_number < (-log_acceptance_probability).exp() {
            *model = new_model;
            println!("Accepted new model with log acceptance probability: {}", log_acceptance_probability.exp());
        }
    }

    model
}

pub fn random_walk_metropolis<'a>(model: &'a mut Model, clouds: &'a Vec<Cloud>, n: usize) -> Vec<Model>{
    // Init the samples
    let mut samples: Vec<Model> = Vec::new();

    // Clone the model for first step
    let mut sample: Model = model.clone();

    // Run a single step metropolis sampler for n iterations
    for _ in 0..n {
        let sample: &mut Model = single_step_metropolis(&mut sample, clouds);
        let record_state: Model = sample.clone();
        samples.push(record_state);
    }

    samples
}