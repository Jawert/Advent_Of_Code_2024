use std::collections::HashMap;

mod utils;

fn main() {
    let parser = utils::NumberParser::new();
    let (mut first_column, mut second_column) = parser.parse_columns();

    first_column.sort_unstable();
    second_column.sort_unstable();

    let total_diff: i32 = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(a, b)| (*a - *b).abs())
        .sum();

    println!("Total difference: {}", total_diff);

    let mut second_column_counts: HashMap<i32, i32> = HashMap::new();

    for &num in second_column.iter() {
        *second_column_counts.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = first_column
        .iter()
        .map(|&value| value * *second_column_counts.get(&value).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity_score);
}
