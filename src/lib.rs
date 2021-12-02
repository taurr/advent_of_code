use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Day1 {
    pub depth: f64,
}

pub fn read_csv<T>(file_path: &Path) -> Result<Vec<T>>
where
    for<'de> T: Deserialize<'de>,
{
    let mut result = vec![];
    let reader = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.set_headers(StringRecord::from(vec!["depth"]));

    for (line, record) in rdr.deserialize().enumerate() {
        let record: T = record.context(line)?;
        result.push(record);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::{fs::File, io::Write};

    #[test]
    fn can_open() -> Result<()> {
        let mut pb = std::env::temp_dir();
        pb.push("csvfile.csv");
        let mut file = File::create(pb.as_path())?;
        for l in ["123", "456"] {
            writeln!(file, "{}", l)?;
        }
        drop(file);

        let r = read_csv::<Day1>(pb.as_path())?;
        assert_eq!(
            vec![Day1 { depth: 123.0_f64 }, Day1 { depth: 456.0_f64 },],
            r
        );
        Ok(())
    }
}
