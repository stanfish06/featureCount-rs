pub mod read_count;
pub mod utils;
use std::process::Command;

fn main() {
    let bam_path = utils::generate_rand_sam_and_bam(5, 5).unwrap();
    let output = Command::new("samtools")
        .arg("index")
        .arg(&bam_path)
        .output();
    let coverage = read_count::count_coverage_at_position(0, 11, &bam_path, 0).unwrap();
    println!("{} reads", coverage);
}
