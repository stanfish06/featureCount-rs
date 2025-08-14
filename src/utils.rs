use chrono::Local;
use std::fmt;
use std::fs::{File, create_dir, create_dir_all, remove_dir_all};
use std::io;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use bam::RecordWriter;
use bam::header::{Header, HeaderEntry};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

use crate::main;

pub enum DnaBase {
    A,
    T,
    C,
    G,
}

impl Distribution<DnaBase> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DnaBase {
        let index: u8 = rng.random_range(0..4);
        match index {
            0 => DnaBase::A,
            1 => DnaBase::T,
            2 => DnaBase::C,
            3 => DnaBase::G,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for DnaBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_char = match self {
            DnaBase::A => 'A',
            DnaBase::T => 'T',
            DnaBase::C => 'C',
            DnaBase::G => 'G',
        };
        write!(f, "{}", base_char)
    }
}

pub fn create_test_folder(
    replace: bool,
    name: Option<&str>,
    create_sub_today: bool,
) -> Result<PathBuf, io::Error> {
    let main_test_folder = Path::new("./featureCount_test");
    let today_date = Local::now().date_naive().to_string();
    match name {
        Some(sub_folder) => {
            let sub_folder_path = main_test_folder.join(sub_folder);
            if replace {
                remove_dir_all(&sub_folder_path)?;
            }
            let target_dir = if create_sub_today {
                sub_folder_path.join(today_date)
            } else {
                sub_folder_path
            };
            if target_dir.is_dir() {
                println!("{} already exists", target_dir.display());
            } else {
                create_dir_all(&target_dir)?;
            }
            Ok(target_dir.to_path_buf())
        }
        None => panic!("Need to specify a folder"),
    }
}

pub fn generate_rand_sam_and_bam(n_base: usize, n_entry: u16) -> Result<(), io::Error> {
    let mut rng = rand::rng();
    let mut sequence: Vec<DnaBase> = Vec::with_capacity(n_base);
    // Creating a header.
    let mut header = Header::new();
    // Header line          "@HD  VN:1.6  SO:Coordinate".
    let mut header_line = HeaderEntry::header_line("1.6".to_string());
    header_line.push(b"SO", "Coordinate".to_string());
    header.push_entry(header_line).unwrap();
    // Reference line       "@SQ  SN:chr1  LN:10000".
    header
        .push_entry(HeaderEntry::ref_sequence("chr1".to_string(), 10000))
        .unwrap();

    let id = Uuid::new_v4().to_string();
    let mut folder = create_test_folder(false, Some("sam_test"), true)?;
    folder.push(format!("{}.sam", id));
    let file = File::create(folder).unwrap();

    let output = io::BufWriter::new(file);
    let mut writer = bam::SamWriter::from_stream(output, header).unwrap();
    for i in 0..n_entry {
        for j in 0..n_base {
            let rand_base: DnaBase = StandardUniform.sample(&mut rng);
            if i > 0 {
                sequence[j] = rand_base;
            } else {
                sequence.push(rand_base);
            }
        }
        let sequence_str: String = sequence.iter().map(|base| base.to_string()).collect();
        let mut record = bam::Record::new();

        record.set_name(format!("Read_{}", i + 1).bytes());
        record.set_ref_id(0);
        record.set_start(10);
        record.flag_mut().set_strand(false);
        record.set_seq_qual(sequence_str.bytes(), sequence.iter().map(|base| 30));
        record.tags_mut().push_num(b"NM", 1);
        writer.write(&record).unwrap();
    }
    writer.finish().unwrap();
    Ok(())
}
