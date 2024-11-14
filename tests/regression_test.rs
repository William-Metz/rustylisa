use rustylisa::data_point::DataPoint;
use rustylisa::simulation_runner::simulation_runner::SimulationRunner;
use rustylisa::test_case::test_case::TestCase;
use serde_json;
use std::fs::File;
use std::path::Path;
use rustylisa::case_supervisor::case_supervisor::CaseSupervisor;
#[test]

fn regression_test() {
    println!("This will be printed with --nocapture");
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

        // Load baseline parameters
        let baseline_params: TestCase = load_json(&baseline_case_dir.join("parameters.json"));

        // Run simulation using the baseline parameters
        let mut case_supervisor = CaseSupervisor::new(baseline_params.clone());
        case_supervisor.run_simulation();

        // Save results (assuming you want to save to the same directory structure)
        // Now compare the results
        let test_case_dir = baseline_dir.join(entry.file_name());
        println!("{:?}", test_case_dir);

        // Load baseline data
        let baseline_data: Vec<DataPoint> =  load_json(&test_case_dir.join("data.json"));
        // Load current test data
        let test_data: Vec<DataPoint> = case_supervisor.wave.spin_evolver.data.clone();
        println!("running now");

        // Compare data
        assert_data_points_equal(&baseline_data, &test_data, "Test case");
        println!("Test case passed.");
    }
}


fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let file = File::open(path).expect(&format!("Failed to open file {:?}", path));
    println!("good");
    serde_json::from_reader(file).expect(&format!("Failed to parse JSON from {:?}", path))
}

fn assert_data_points_equal(baseline: &[DataPoint], current: &[DataPoint], case_name: &str) {
    assert_eq!(
        baseline.len(),
        current.len(),
        "Data length mismatch in {}",
        case_name
    );

    for (i, (b_point, c_point)) in baseline.iter().zip(current.iter()).enumerate() {
        // Implement a tolerance for floating-point comparison
        assert!(
            approximately_equal(b_point, c_point),
            "Data mismatch at index {} in {}",
            i,
            case_name
        );
    }
}

fn approximately_equal(a: &DataPoint, b: &DataPoint) -> bool {
    // Define acceptable tolerance
    let tolerance = 1e-6;

    // Compare fields with tolerance
    // Example:
    (a.hp - b.hp).abs() < tolerance && (a.hx - b.hx).abs() < tolerance
    // Continue for other fields...
}
