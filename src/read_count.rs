use bam::IndexedReader;
use bam::Region;
use std::io;
use std::path::Path;
pub fn count_coverage_at_position(
    chr_id: u32,
    pos: u32,
    bam_path: &Path,
    min_mapq: u8,
) -> Result<usize, io::Error> {
    let mut reader = IndexedReader::from_path(bam_path).unwrap();
    let mut depth = 0;
    for record in reader.fetch_by(&Region::new(chr_id, pos, pos + 1), move |record| {
        record.mapq() >= min_mapq
    })? {
        depth += 1;
    }
    Ok(depth)
}
