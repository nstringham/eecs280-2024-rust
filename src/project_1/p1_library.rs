use csv::{Reader, StringRecord};
use rand::prelude::*;

/// Extracts two columns of data from a delimited file (e.g. .csv, .tsv),
/// omitting rows where either column has a missing or invalid value. The
/// columns are returned as a pair of vectors. Prints errors to stdout and
/// exits with non-zero status on errors.
pub fn extract_columns(
    filename: &str,
    column_name_1: &str,
    column_name_2: &str,
) -> (Vec<f64>, Vec<f64>) {
    let mut csv = Reader::from_path(filename).unwrap();

    let headers = csv.headers().unwrap();

    fn find_column_index(headers: &StringRecord, column_name: &str) -> usize {
        headers
            .iter()
            .enumerate()
            .find(|(_index, name)| *name == column_name)
            .unwrap_or_else(|| panic!("missing column {column_name}"))
            .0
    }

    let column_index_1 = find_column_index(headers, column_name_1);
    let column_index_2 = find_column_index(headers, column_name_2);

    csv.records()
        .filter_map(|record| {
            let record = record.ok()?;
            Some((
                record.get(column_index_1)?.parse::<f64>().ok()?,
                record.get(column_index_2)?.parse::<f64>().ok()?,
            ))
        })
        .unzip()
}

/// Returns a bootstrap resampling of the given data, which has the
/// same number of elements as the original data vector. The values
/// of those elements are determined by pseudorandomly selecting from
/// the original data, with replacement, where the sample_num is used
/// to seed a pseudorandom number generator. The resample generated by
/// this function is uniquely determined by the values in the original
/// data vector (regardless of their ordering) and the provided
/// sample_num, and is consistent across platforms.
pub fn bootstrap_resample(mut data: Vec<f64>, sample_num: usize) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }

    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let seed = data.iter().fold(sample_num as u64, |accumulator, x| {
        accumulator ^ u64::from_be_bytes(x.to_be_bytes())
    });

    let mut rng = StdRng::seed_from_u64(seed);

    (0..data.len())
        .map(|_| data.choose(&mut rng).unwrap())
        .cloned()
        .collect()
}
