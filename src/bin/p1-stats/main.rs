mod p1_library;
mod stats;

use std::{env, process::ExitCode};

use p1_library::*;
use stats::*;

/// Prints descriptive statistics for the given data
fn print_descriptive_stats(data: &[f64]) -> () {
    todo!()
}

/// Returns an approximation of the sampling distribution of the
/// difference in means between groups A and B, represented as a
/// vector of the computed difference in means for 1000 bootstrap
/// resamples of original samples data_A and data_B.
fn mean_diff_sampling_distribution(data_a: &[f64], data_b: &[f64]) -> Vec<f64> {
    todo!()
}

/// Returns a confidence interval for the data in v of the given
/// width, centered on the 50th percentile. The interval is represented
/// as a pair of upper and lower bounds. For example, the bounds on a
/// confidence interval with width 0.8 are the 10th and 90th percentiles.
fn confidence_interval(v: &[f64], width: f64) -> (f64, f64) {
    todo!()
}

fn two_sample_analysis(
    file_name: &str,
    filter_column_name: &str,
    filter_val_a: f64,
    filter_val_b: f64,
    data_column_name: &str,
) -> () {
    // Extract data from the input file, removing rows with missing data.
    // filter_data and data have type vector<double>
    let (filter_data, data) = extract_columns(file_name, filter_column_name, data_column_name);

    // Filter data into groups A and B based on given criteria
    let data_a = filter(&data, &filter_data, filter_val_a);
    let data_b = filter(&data, &filter_data, filter_val_b);

    // Print descriptive statistics for group A
    println!("Group A: {data_column_name} | {filter_column_name} = {filter_val_a}");
    print_descriptive_stats(&data_a);
    println!();

    // Print descriptive statistics for group B
    println!("Group B: {data_column_name} | {filter_column_name} = {filter_val_b}");
    print_descriptive_stats(&data_b);
    println!();

    // Use bootstrap resampling to approximate the sampling distribution of the
    // difference in means between groups A and B and compute a confidence interval.
    let mean_diffs = mean_diff_sampling_distribution(&data_a, &data_b);
    let ci_95 = confidence_interval(&mean_diffs, 0.95);

    // Print confidence interval for the difference in means
    println!(
        "Confidence interval for mean({data_column_name} | A) - mean({data_column_name} | B):"
    );
    println!("  95% [{}, {}]", ci_95.0, ci_95.1);
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        // No command-line arguments, use default values for cats.csv sample data
        two_sample_analysis("sample-data/cats.csv", "food", 1.0, 2.0, "weight");
        ExitCode::SUCCESS
    } else if args.len() == 5 {
        // Use custom values from command-line arguments
        two_sample_analysis(
            &args[0],
            &args[1],
            args[2].parse().unwrap(),
            args[3].parse().unwrap(),
            &args[4],
        );
        ExitCode::SUCCESS
    } else {
        eprintln!("Error: requires exactly zero or five command-line arguments.");
        eprintln!("Usage:");
        eprintln!("  ./two_sample.exe");
        eprintln!("  ./two_sample.exe file_name filter_column_name filter_val_a filter_val_b data_column_name");
        ExitCode::FAILURE
    }
}
