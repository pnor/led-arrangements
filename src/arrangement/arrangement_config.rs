use core::fmt;
use csv::StringRecord;
use std::string::String;
use std::{error::Error, fs::File};

pub struct ArrangementConfig<const N: usize> {
    pub light_locations: Vec<([f64; N], usize)>,
}

impl<const N: usize> ArrangementConfig<N> {
    /// Reads the csv file at `file_path` to determine the lcoation of each light index.
    /// The csv file should be formatted with the lcoation first, then the index
    /// Example: light index 0 at (0.5, 0.2) would be
    /// 0.5,0.2,0
    /// Example: light index 1 at (0.5, 0.2, 0.1, 0.8) would be
    /// 0.5,0.2,0.1,0.8,1
    pub fn from_csv(file_path: &String) -> Result<Self, ArrangementConfigError> {
        let file = File::open(file_path);
        let mut light_locations: Vec<([f64; N], usize)> = vec![];

        if let Ok(file) = file {
            let mut reader = csv::Reader::from_reader(file);
            for result in reader.records() {
                if let Ok(record) = result {
                    light_locations.push(parse_record(&record)?)
                } else {
                    return Err(ArrangementConfigError {
                        reason: "Error reading row in csv".to_string(),
                    });
                }
            }
            return Ok(Self { light_locations });
        } else {
            return Err(ArrangementConfigError {
                reason: format!("Unable to open file: {}", file_path),
            });
        }
    }
}

fn parse_record<const N: usize>(
    record: &StringRecord,
) -> Result<([f64; N], usize), ArrangementConfigError> {
    if record.len() != N + 1 {
        return Err(ArrangementConfigError {
            reason: format!(
                "Row in csv had wrong number of
        elements; {} instead of {}",
                record.len(),
                N + 1
            ),
        });
    }

    let mut pos = [0.0; N];

    for i in 0..N {
        if let Ok(val) = record.get(i).unwrap_or("").parse::<f64>() {
            pos[i] = val;
        } else {
            return Err(ArrangementConfigError {
                reason: format!("Unable to access and parse field {} of row", i),
            });
        }
    }

    if let Ok(index) = record
        .get(N)
        .ok_or(ArrangementConfigError {
            reason: format!("Unable to access field {} of row", N),
        })?
        .parse::<usize>()
    {
        return Ok((pos, index));
    } else {
        return Err(ArrangementConfigError {
            reason: "Unable to convert final field to index".to_string(),
        });
    }
}

#[derive(Debug)]
pub struct ArrangementConfigError {
    reason: String,
}

impl ArrangementConfigError {
    pub fn new(reason: String) -> Self {
        ArrangementConfigError { reason }
    }

    pub fn reason(&self) -> String {
        String::from(&self.reason)
    }
}

impl Error for ArrangementConfigError {}

impl fmt::Display for ArrangementConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to create ArrangementConfig: {}", self.reason)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_csv() -> Result<(), Box<dyn Error>> {
        let arr = ArrangementConfig::<2>::from_csv(&"./test_files/test.csv".to_string())?;
        let mut loc = arr.light_locations;
        loc.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        assert_eq!(loc[0], ([0.0, 0.0], 0));
        assert_eq!(loc[1], ([0.1, 0.1], 1));
        assert_eq!(loc[2], ([0.3, 0.1], 2));
        assert_eq!(loc[3], ([1.0, 0.3], 3));
        assert_eq!(loc[4], ([1.0, 1.0], 4));

        return Ok(());
    }
}
