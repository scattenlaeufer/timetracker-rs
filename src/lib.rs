use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct WorkSession {
    start: DateTime<Local>,
    stop: Option<DateTime<Local>>,
    description: String,
}

impl WorkSession {
    fn start_new_work_session(start: DateTime<Local>, description: String) -> WorkSession {
        WorkSession {
            start,
            description,
            stop: None,
        }
    }
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

    fn load(path: &Path) -> Result<TimeSheet, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(&path)?;
        let reader = BufReader::new(&file);
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line?);
        }
        let json_string = lines.join("\n");
        match TimeSheet::from_json(json_string) {
            Ok(t) => Ok(t),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl TimeSheet {
    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }

    fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(&path)?;
        let mut writer = BufWriter::new(&file);
        write!(&mut writer, "{}", &self.to_json()?)?;
        Ok(())
    }
}

#[derive(Debug)]
struct TimeSheetError {
    message: String,
}

impl std::error::Error for TimeSheetError {}

impl std::fmt::Display for TimeSheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl TimeSheetError {
    fn new(message: String) -> TimeSheetError {
        TimeSheetError { message }
    }
}

pub fn initialize_project(
    name: String,
    hourly_rate: f32,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Initializing Project {} with an hourly rate of {}€",
        name, hourly_rate
    );
    let time_sheet = TimeSheet::new(name, hourly_rate);
    time_sheet.save(path)?;
    Ok(())
}

pub fn start_working_session(description: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Local::now();
    let mut desc = String::new();
    let path = Path::new("time_sheet.json");
    let mut time_sheet = TimeSheet::load(&path)?;
    if let Some(s) = time_sheet.work_sessions.last() {
        match s.stop {
            None => {
                return Err(Box::new(TimeSheetError::new(String::from(
                    "Last work session not finished!",
                ))));
            }
            Some(_) => (),
        }
    };
    match description {
        Some(d) => {
            desc.push_str(d);
            println!("Start working on {} at {:?}", desc, start_time);
        }
        None => println!("Start working at {:?}", start_time),
    };
    time_sheet
        .work_sessions
        .push(WorkSession::start_new_work_session(start_time, desc));
    time_sheet.save(&path)?;
    println!("{:#?}", time_sheet);
    Ok(())
}

pub fn stop_working_session(description: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let stop_time = Local::now();
    let mut desc = String::new();
    let path = Path::new("time_sheet.json");
    let mut time_sheet = TimeSheet::load(&path).unwrap();
    match time_sheet.work_sessions.last() {
        Some(s) => match s.stop {
            None => (),
            Some(_) => {
                return Err(Box::new(TimeSheetError::new(String::from(
                    "No unfinished work session found to stop!",
                ))));
            }
        },
        None => {
            return Err(Box::new(TimeSheetError::new(String::from(
                "No unfinished work session found to stop!",
            ))));
        }
    }
    match description {
        Some(d) => {
            desc.push_str(d);
            println!("Stop working on {} at {:?}", desc, stop_time);
        }
        None => println!("Stop working at {:?}", stop_time),
    }
    //time_sheet.work_sessions.last().unwrap().stop = Some(stop_time);
    let mut last_work_session = time_sheet.work_sessions.pop().unwrap();
    last_work_session.stop = Some(stop_time);
    last_work_session.description = desc;
    time_sheet.work_sessions.push(last_work_session);
    time_sheet.save(&path).unwrap();
    println!("{:#?}", time_sheet);
    Ok(())
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
