
RustyLISA
RustyLISA is a Rust library designed to simulate gravitational wave signals, particularly those detected by the Laser Interferometer Space Antenna (LISA). Built for performance and extensibility, RustyLISA leverages the power of Rust to run efficient, parallelized simulations. This library is useful for researchers, engineers, and enthusiasts interested in gravitational wave astrophysics.

Table of Contents
Installation
Getting Started
Usage
Parameters
Contributing
License
Installation
To include RustyLISA in your Rust project, add it as a dependency in your Cargo.toml:

toml
Copy code
[dependencies]
rustylisa = { git = "https://github.com/William-Metz/rustylisa.git" }
After that, run:

sh
Copy code
cargo build
To build RustyLISA as a standalone library, clone the repository and use:

sh
Copy code
git clone https://github.com/William-Metz/rustylisa.git
cd rustylisa
cargo build --release
Getting Started
To start using RustyLISA, import the library in your Rust code:

rust
Copy code
use rustylisa::simulation;
You can then create a new simulation instance and configure it according to your needs.

Basic Example
Here’s a minimal example to initialize a simulation:

rust
Copy code
use rustylisa::simulation::SimulationConfig;

fn main() {
    let config = SimulationConfig::new()
        .set_mass(1.4) // Mass of source (solar masses)
        .set_distance(10.0) // Distance from source (Mpc)
        .set_time_duration(1.0); // Time duration of the simulation (years)

    let simulation = rustylisa::run_simulation(config);
    println!("Simulation results: {:?}", simulation);
}
Usage
The library provides flexible configuration options to tailor simulations for different types of gravitational wave sources. You can adjust parameters to define characteristics of the wave source, the detector, and the environmental conditions.

To run a simulation:

Define parameters for the wave source.
Configure LISA’s settings.
Execute the simulation and analyze results.
For additional customization, refer to the parameter list below.

Parameters
Here’s a description of the available parameters to configure the RustyLISA simulation:

Parameter	Type	Description	Default
mass	f64	Mass of the gravitational wave source in solar masses.	1.4
distance	f64	Distance from LISA to the source in megaparsecs (Mpc).	10.0
time_duration	f64	Duration of the simulation in years.	1.0
sampling_rate	f64	Rate at which signals are sampled in Hertz (Hz).	1.0
frequency_range	(f64, f64)	Frequency range of the gravitational waves to detect (min, max) in Hertz.	(1e-4, 1.0)
inclination_angle	f64	Angle of the source’s orbit relative to the detector plane in degrees.	0.0
phase_shift	f64	Phase shift of the signal at the beginning of the observation period.	0.0
initial_position	(f64, f64, f64)	Initial (x, y, z) position of the source in space (in AU, Astronomical Units).	(0.0, 0.0, 0.0)
velocity	(f64, f64, f64)	Initial velocity vector (vx, vy, vz) of the source in AU per year.	(0.0, 0.0, 0.0)
output_precision	u32	Number of decimal places for output precision.	5
noise_level	f64	Background noise level for signal processing, useful for simulating real-world interference.	0.01
Detailed Parameter Explanations
mass: Mass of the source, which affects the gravitational wave’s amplitude.
distance: Defines the distance to the source; further sources will result in weaker signals.
time_duration: Length of the simulation in years; longer durations require more computational resources.
sampling_rate: Determines the number of samples per second; higher rates yield more precise signals.
frequency_range: Defines the range of wave frequencies to detect; useful for isolating signals.
inclination_angle: Adjusts the orientation of the source’s orbit relative to LISA.
phase_shift: Sets the initial phase of the waveform at the start of the simulation.
initial_position: Starting position of the source, allowing for spatial modeling of source trajectories.
velocity: Initial velocity vector, enabling simulations of moving wave sources.
output_precision: Precision of the output values, affecting storage size and readability.
noise_level: Adds synthetic noise to the signal, ideal for testing noise-cancellation algorithms.
Contributing
Contributions are welcome! Please fork the repository and submit a pull request, or open an issue for discussion. Ensure that any changes follow Rust coding standards and are tested thoroughly.

License
This project is licensed under the MIT License. See the LICENSE file for more details.

