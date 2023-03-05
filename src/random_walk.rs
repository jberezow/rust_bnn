// Random Walk Metropolis Sampler

use na::Vector2;
use crate::blr::Model;
use crate::point2::Point2;
use crate::cloud::Cloud;

pub fn random_walk_metropolis(model: &mut Model, clouds: Vec<Cloud>, n: usize, step_size: f64) -> Vec<&mut Model> {
    // Init the samples
    let mut samples = Vec::new();

    // Condense clouds to one Vec of Point2
    let mut points = Vec::new();
    for cloud in clouds.iter() {
        for point in cloud.points.as_ref().unwrap().iter() {
            points.push(point);
        }
    }

    // Generate n samples
    for _ in 0..n {
        // Generate a new model
        let new_model: Model = Model::default();

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
            model = new_model;
        }
        else if log_acceptance_probability < 0.0 {
            // Generate a random number between 0 and 1
            let random_number = rand::random::<f64>();

            // Calculate the acceptance probability
            let acceptance_probability = (log_acceptance_probability).exp();

            // Accept the new model if the acceptance probability is greater than the random number
            if acceptance_probability > random_number {
                model = new_model;
            }
        }

        // Add the model to the samples
        samples.push(model);
    }

    samples
}