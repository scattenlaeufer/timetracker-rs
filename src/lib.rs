use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct WorkSession {
    start: DateTime<Local>,
    end: Option<DateTime<Local>>,
    description: String,
}

/*
impl proptest::arbitrary::Arbitrary for WorkSession {
    fn arbtrary_with(
        args: <Type as proptest::arbitrary::traits::Arbitrary>::Parameters,
    ) -> <Type as proptest::arbitrary::traits::Arbitrary>::Strategy {
        ()
    }
}
*/

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TimeSheet {
    project_name: String,
    hourly_rate: f32,
    work_sessions: Vec<WorkSession>,
}

impl TimeSheet {
    fn new(project_name: String, hourly_rate: f32) -> TimeSheet {
        TimeSheet {
            project_name,
            hourly_rate,
            work_sessions: Vec::new(),
        }
    }

    fn from_json(json_string: String) -> serde_json::Result<TimeSheet> {
        serde_json::from_str(&json_string)
    }
}

impl TimeSheet {
    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

pub fn initialize_project(name: String, hourly_rate: f32, path: &Path) {
    println!(
        "Initializing Project {} with an hourly rate of {}€",
        name, hourly_rate
    );
    let time_sheet = TimeSheet::new(name, hourly_rate);
    let time_sheet_file = std::fs::File::create(&path).unwrap();
    let mut writer = BufWriter::new(&time_sheet_file);
    write!(&mut writer, "{}", time_sheet.to_json().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn dummy_test() {
        assert!(true);
    }

    /*
    prop_compose! {
        fn compose_work_sessions(max_length: usize)(work_sessions in any_with::<Vec<WorkSession>>(proptest::collection::size_range(max_length).lift())) -> Vec<WorkSession> {
            work_sessions
        }
    }
    */

    proptest! {
        #[test]
        fn test_time_sheet_creation(project_name in "\\PC*", hourly_rate: f32) {
            let time_sheet = TimeSheet::new(project_name.clone(), hourly_rate);
            prop_assert_eq!(time_sheet.project_name, project_name);
            prop_assert_eq!(time_sheet.hourly_rate, hourly_rate);
            assert_eq!(time_sheet.work_sessions.len(), 0);
        }
    }
}
