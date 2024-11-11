use rustylisa::data_point::DataPoint;
use rustylisa::simulation_runner::simulation_runner::SimulationRunner;
use rustylisa::test_case::test_case::TestCase;
use serde_json;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::runtime::Runtime;
#[test]
fn regression_test() {
    let baseline_dir = Path::new("results/baseline/");
    let test_cases_dir = Path::new("results/test_cases/");
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    let runtime = Arc::new(runtime);

    // Collect all test cases from the baseline parameters
    let mut test_cases = Vec::new();
    let mut case_names = Vec::new();

    for entry in baseline_dir
        .read_dir()
        .expect("Failed to read baseline directory")
    {
        let entry = entry.expect("Failed to read directory entry");
        let case_name = entry.file_name();
        let baseline_case_dir = baseline_dir.join(&case_name);

        // Load baseline parameters
        let baseline_params: TestCase = load_json(&baseline_case_dir.join("parameters.json"));
        test_cases.push(baseline_params);
        case_names.push(case_name);
    }

    // Run simulations using the baseline parameters
    let simulation_runner = SimulationRunner::new(test_cases.clone(), runtime.clone());
    simulation_runner.run_all_simulations(true);

    // Wait until all simulations are complete
    runtime.block_on(async {
        while simulation_runner.simulations_running.load(Ordering::SeqCst) > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    // Now compare the results
    for case_name in case_names {
        let baseline_case_dir = baseline_dir.join(&case_name);
        let test_case_dir = test_cases_dir.join(&case_name);

        // Load baseline data
        let baseline_data: Vec<DataPoint> = load_json(&baseline_case_dir.join("data.json"));
        let baseline_params: TestCase = load_json(&baseline_case_dir.join("parameters.json"));

        // Load current test data
        let test_data: Vec<DataPoint> = load_json(&test_case_dir.join("data.json"));
        let test_params: TestCase = load_json(&test_case_dir.join("parameters.json"));

        // Compare parameters
        assert_eq!(
            baseline_params, test_params,
            "Parameters mismatch in {:?}",
            case_name
        );

        // Compare data
        assert_data_points_equal(&baseline_data, &test_data, &case_name.to_string_lossy());
        println!("Test case {:?} passed.", case_name);
    }
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let file = File::open(path).expect(&format!("Failed to open file {:?}", path));
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
