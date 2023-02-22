use csv::StringRecord;
use std::fs::File;
use std::string::String;

use crate::LightArrangementError;

pub struct ArrangementConfig<const N: usize> {
    pub light_locations: Vec<([f64; N], usize)>,
    pub number_children_for_division: usize,
}

impl<const N: usize> ArrangementConfig<N> {
    /// Reads the csv file at `file_path` to determine the lcoation of each light index.
    /// The csv file should be formatted with the lcoation first, then the index
    /// Example: light index 0 at (0.5, 0.2) would be
    /// 0.5,0.2,0
    /// Example: light index 1 at (0.5, 0.2, 0.1, 0.8) would be
    /// 0.5,0.2,0.1,0.8,1
    pub fn from_csv(
        file_path: &String,
        number_children_for_division: usize,
    ) -> Result<Self, LightArrangementError> {
        let file = File::open(file_path);
        let mut light_locations: Vec<([f64; N], usize)> = vec![];

        if let Ok(file) = file {
            let mut reader = csv::Reader::from_reader(file);
            for result in reader.records() {
                if let Ok(record) = result {
                    light_locations.push(parse_record(&record)?)
                } else {
                    return Err(LightArrangementError::new(
                        "Error reading row in csv".to_string(),
                    ));
                }
            }
            return Ok(Self {
                light_locations,
                number_children_for_division,
            });
        } else {
            return Err(LightArrangementError::new(format!(
                "Unable to open file: {}",
                file_path
            )));
        }
    }
}

fn parse_record<const N: usize>(
    record: &StringRecord,
) -> Result<([f64; N], usize), LightArrangementError> {
    if record.len() != N + 1 {
        return Err(LightArrangementError::new(format!(
            "Row in csv had wrong number of
        elements; {} instead of {}",
            record.len(),
            N + 1
        )));
    }

    let mut pos = [0.0; N];

    for i in 0..N {
        if let Ok(val) = record.get(i).unwrap_or("").parse::<f64>() {
            pos[i] = val;
        } else {
            return Err(LightArrangementError::new(format!(
                "Unable to access and parse field {} of row",
                i
            )));
        }
    }

    if let Ok(index) = record
        .get(N)
        .ok_or(LightArrangementError::new(format!(
            "Unable to access field {} of row",
            N
        )))?
        .parse::<usize>()
    {
        return Ok((pos, index));
    } else {
        return Err(LightArrangementError::new(
            "Unable to convert final field to index".to_string(),
        ));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_csv() -> Result<(), Box<dyn Error>> {
        let arr = ArrangementConfig::<2>::from_csv(&"./test_files/test.csv".to_string(), 1)?;
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
