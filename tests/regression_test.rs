use rand_distr::num_traits::float::FloatCore;
use rand_distr::num_traits::Float;
use rustylisa::case_supervisor::case_supervisor::CaseSupervisor;
use rustylisa::data_point::DataPoint;
use rustylisa::simulation_runner::simulation_runner::SimulationRunner;
use rustylisa::test_case::test_case::TestCase;
use std::fs::File;
use std::path::Path;

use csv::ReaderBuilder;
use serde::Deserialize;

#[test]
fn regression_test() {
    println!("Starting regression tests...");

    let baseline_dir = Path::new("results/baseline/");

    // Collect all test case directories from the baseline directory
    let entries = baseline_dir
        .read_dir()
        .expect("Failed to read baseline directory")
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to collect directory entries");

    // Process each test case one at a time
    for entry in entries {
        let baseline_case_dir = entry.path();

        // Load baseline parameters from JSON
        let baseline_params: TestCase = load_json(&baseline_case_dir.join("parameters.json"));

        // Run simulation using the baseline parameters
        let mut case_supervisor = CaseSupervisor::new(baseline_params.clone());
        case_supervisor.run_simulation();

        // Save results (assuming you want to save to the same directory structure)
        let test_case_dir = baseline_dir.join(entry.file_name());

        // Load baseline data from CSV
        let baseline_data: Vec<DataPoint> = load_csv(&test_case_dir.join("data.csv"));

        // Load current test data from the simulation
        let test_data: Vec<DataPoint> = case_supervisor.wave.spin_evolver.data.clone();

        // Compare data
        assert_data_points_equal(&baseline_data, &test_data, "Test case");
    }
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let file = File::open(path).expect(&format!("Failed to open file {:?}", path));
    serde_json::from_reader(file).expect(&format!("Failed to parse JSON from {:?}", path))
}

fn load_csv<T>(path: &Path) -> Vec<T>
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(path).expect(&format!("Failed to open file {:?}", path));
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    reader
        .deserialize()
        .map(|result| result.expect("Failed to parse CSV record"))
        .collect()
}

fn assert_data_points_equal(baseline: &[DataPoint], current: &[DataPoint], case_name: &str) {
    assert_eq!(
        baseline.len(),
        current.len(),
        "Data length mismatch in {}",
        case_name
    );

    for i in (0..baseline.len()).step_by(3) {
        let b_point = &baseline[i];
        let c_point = &current[i];

        // Implement a tolerance for floating-point comparison
        assert!(
            approximately_equal(b_point, c_point),
            "Data mismatch at index {} in {}, b:{:?}, c:{:?}",
            i,
            case_name,
            b_point,
            c_point
        );
    }
}

fn approximately_equal(a: &DataPoint, b: &DataPoint) -> bool {
    // Define acceptable tolerance
    let tolerance = 1e-6;

    // Compare `hp` field
    let hp_close = if a.hp.is_nan() && b.hp.is_nan() {
        true // Both are NaN, consider them equal
    } else if a.hp.is_nan() || b.hp.is_nan() {
        false // One is NaN and the other isn't
    } else {
        (a.hp - b.hp).abs() < tolerance
    };

    // Compare `hx` field
    let hx_close = if a.hx.is_nan() && b.hx.is_nan() {
        true
    } else if a.hx.is_nan() || b.hx.is_nan() {
        false
    } else {
        (a.hx - b.hx).abs() < tolerance
    };

    // Combine all comparisons
    hp_close && hx_close // && other field comparisons
}
