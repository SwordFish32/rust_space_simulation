#[allow(unused_imports)]
use rand::Rng;
/* use std::f64::consts::PI; /* Could use it latter */ */ 

fn main() {
    let _rng = rand::thread_rng();

    // Define spaceship properties
    let mut position = (0.0, 0.0); // x, y coordinates (in meters for this example)
    let mass = 100.0; // kg (consider a realistic mass for a spacecraft)
    let sail_area = 100.0; // m^2 (typical solar sail sizes can range from tens to thousands)

    // Simulation parameters
    let light_pressure = 0.1; // Newtons/meter^2 (adjustable for visual interest)  - Consider renaming to force_per_area
    let time_step = 0.1; // seconds (consider adjusting for smoother motion)
    let simulation_time = 30.0; // seconds (simulation duration)

    // Track traveled distance
    let mut total_distance = 0.0;

    // Simulation loop
    let mut elapsed_time = 0.0;
    while elapsed_time < simulation_time {
        // Simulate sunlight pressure force (assuming sail is perpendicular to sunlight)
        let force = (light_pressure * sail_area, 0.0); // Newtons (positive x-axis)

        // Update position based on force and time step
        let (new_x, new_y) = update_position(position, force, mass, time_step);
        position = (new_x, new_y);

        // Calculate distance traveled in this step
        let distance_traveled = calculate_distance(position, (0.0, 0.0)); // Origin is assumed as starting point

        // Update total distance traveled
        total_distance += distance_traveled;

        // Print informative message
        println!(
            "Simulation time: {:.2} seconds, Distance Traveled: {:.2} meters, Current Position: ({:.2}, {:.2}) meters",
            elapsed_time, total_distance, position.0, position.1
        );

        // Update elapsed time
        elapsed_time += time_step;
    }

    // End message with additional considerations
    println!("Simulation complete! Your spacecraft has traveled a total distance of approximately {:.2} meters.", total_distance);
    println!(
        "Remember, this is a simplified simulation. Real solar sail missions involve complex orbital mechanics and consider factors like:"
    );
    println!("  - The inverse-square law: Sunlight pressure weakens as the distance from the sun squares.");
    println!("  - The angle of the sail relative to the sunlight, which affects the force exerted.");
    println!("  - Trajectory adjustments using light pressure for course correction.");
    println!("  - Sophisticated navigation techniques to avoid obstacles and reach destinations.");
    println!("Thank you for exploring space with solar sails!");
}

// Function to update position based on force, mass, and time step
fn update_position(position: (f64, f64), force: (f64, f64), mass: f64, time_step: f64) -> (f64, f64) {
    let (x, y) = position;
    let (force_x, force_y) = force;
    let new_x = x + force_x / mass * time_step;
    let new_y = y + force_y / mass * time_step;
    return (new_x, new_y);
}

// Function to calculate distance between two points (basic distance formula)
fn calculate_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let distance = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
    return distance;
}
