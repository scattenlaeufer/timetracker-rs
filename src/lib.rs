use chrono::prelude::*;
use prettytable::{cell, format, row, Table};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M";

/// A enum to represent possible errors within a timetracker
#[derive(Debug)]
pub enum TimetrackerError {
    Subproject(String),
    Activity(String),
    IOError(String),
    SerdeJSON(String),
    ChronoParse(String),
    TimeSheet(String),
}

impl std::error::Error for TimetrackerError {}

impl fmt::Display for TimetrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimetrackerError::Subproject(e) => write!(f, "Subproject Error: {}", e),
            TimetrackerError::Activity(e) => write!(f, "Activity Error: {}", e),
            TimetrackerError::IOError(e) => write!(f, "IO Error: {}", e),
            TimetrackerError::SerdeJSON(e) => write!(f, "Serde JSON Error: {}", e),
            TimetrackerError::ChronoParse(e) => write!(f, "Chrono Parse Error: {}", e),
            TimetrackerError::TimeSheet(e) => write!(f, "TimeSheet Error: {}", e),
        }
    }
}

impl From<std::io::Error> for TimetrackerError {
    fn from(error: std::io::Error) -> Self {
        TimetrackerError::IOError(error.to_string())
    }
}

impl From<serde_json::Error> for TimetrackerError {
    fn from(error: serde_json::Error) -> Self {
        TimetrackerError::SerdeJSON(error.to_string())
    }
}

impl From<chrono::ParseError> for TimetrackerError {
    fn from(error: chrono::ParseError) -> Self {
        TimetrackerError::ChronoParse(error.to_string())
    }
}

#[derive(Serialize, Deserialize, Eq, Debug)]
struct SubProject {
    id: usize,
    name: String,
    description: String,
}

impl PartialEq for SubProject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description && self.id == other.id
    }
}

impl Ord for SubProject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for SubProject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl SubProject {
    fn new(id: usize, name: String, description: String) -> SubProject {
        SubProject {
            id,
            name,
            description,
        }
    }
}

#[derive(Serialize, Deserialize, Eq, Debug)]
struct WorkSession {
    start: DateTime<Local>,
    stop: Option<DateTime<Local>>,
    description: String,
    #[serde(default)]
    homeoffice: bool,
}

impl PartialEq for WorkSession {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.stop == other.stop
            && self.description == other.description
    }
}

impl Ord for WorkSession {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for WorkSession {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl WorkSession {
    fn new(
        start: DateTime<Local>,
        stop: Option<DateTime<Local>>,
        description: String,
        homeoffice: bool,
    ) -> WorkSession {
        WorkSession {
            start,
            stop,
            description,
            homeoffice,
        }
    }

    fn start_new_work_session(
        start: DateTime<Local>,
        description: String,
        homeoffice: bool,
    ) -> WorkSession {
        WorkSession {
            start,
            description,
            homeoffice,
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
    hourly_rate: Option<f32>,
    work_sessions: Vec<WorkSession>,
    #[serde(default)]
    subprojects: Vec<SubProject>,
}

impl TimeSheet {
    fn new(project_name: String, hourly_rate: Option<f32>) -> TimeSheet {
        TimeSheet {
            project_name,
            hourly_rate,
            work_sessions: Vec::new(),
            subprojects: Vec::new(),
        }
    }

    fn from_json(json_string: String) -> serde_json::Result<TimeSheet> {
        serde_json::from_str(&json_string)
    }

    fn load(path: &Path) -> Result<TimeSheet, TimetrackerError> {
        let file = std::fs::File::open(&path)?;
        let reader = BufReader::new(&file);
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line?);
        }
        let json_string = lines.join("\n");
        Ok(TimeSheet::from_json(json_string)?)
    }
}

impl TimeSheet {
    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }

    fn save(&self, path: &Path) -> Result<(), TimetrackerError> {
        let file = std::fs::File::create(&path)?;
        let mut writer = BufWriter::new(&file);
        write!(&mut writer, "{}", &self.to_json()?)?;
        Ok(())
    }
}

fn split_description_string(desc_string: &str, max_line_length: usize) -> String {
    let desc_split = desc_string.split(' ');
    let mut lines_vec = vec![];
    let mut line_vec = vec![];
    for word in desc_split {
        if line_vec.join(" ").graphemes(true).count() + word.graphemes(true).count()
            > max_line_length
        {
            lines_vec.push(line_vec.join(" "));
            line_vec.clear();
        }
        line_vec.push(word);
    }
    lines_vec.push(line_vec.join(" "));
    lines_vec.join("\n")
}

pub fn initialize_project(
    name: String,
    hourly_rate: Option<f32>,
    path: &Path,
) -> Result<(), TimetrackerError> {
    println!(
        "Initializing Project {} with an hourly rate of {:.02}€",
        name,
        hourly_rate.unwrap_or(0f32)
    );
    let time_sheet = TimeSheet::new(name, hourly_rate);
    time_sheet.save(path)?;
    Ok(())
}

pub fn start_working_session(
    description: Option<&str>,
    homeoffice: bool,
) -> Result<(), TimetrackerError> {
    let start_time = Local::now();
    let mut desc = String::new();
    let path = Path::new("time_sheet.json");
    let mut time_sheet = TimeSheet::load(&path)?;
    if let Some(s) = time_sheet.work_sessions.last() {
        match s.stop {
            None => {
                return Err(TimetrackerError::TimeSheet(String::from(
                    "Last work session not finished!",
                )));
            }
            Some(_) => (),
        }
    };
    match description {
        Some(d) => {
            desc.push_str(d);
            println!(
                "Start working on {} at {}",
                desc,
                start_time.format(DATETIME_FORMAT)
            );
        }
        None => println!("Start working at {}", start_time.format(DATETIME_FORMAT)),
    };
    time_sheet
        .work_sessions
        .push(WorkSession::start_new_work_session(
            start_time, desc, homeoffice,
        ));
    time_sheet.save(&path)?;
    Ok(())
}

pub fn stop_working_session(
    description: Option<&str>,
    homeoffice: bool,
) -> Result<(), TimetrackerError> {
    let stop_time = Local::now();
    let mut desc = String::new();
    let path = Path::new("time_sheet.json");
    let mut time_sheet = TimeSheet::load(&path).unwrap();
    match time_sheet.work_sessions.last() {
        Some(s) => match s.stop {
            None => (),
            Some(_) => {
                return Err(TimetrackerError::TimeSheet(String::from(
                    "No unfinished work session found to stop!",
                )));
            }
        },
        None => {
            return Err(TimetrackerError::TimeSheet(String::from(
                "No unfinished work session found to stop!",
            )));
        }
    }
    match description {
        Some(d) => {
            desc.push_str(d);
            println!(
                "Stop working on {} at {}",
                desc,
                stop_time.format(DATETIME_FORMAT)
            );
        }
        None => println!("Stop working at {}", stop_time.format(DATETIME_FORMAT)),
    }
    //time_sheet.work_sessions.last().unwrap().stop = Some(stop_time);
    let mut last_work_session = time_sheet.work_sessions.pop().unwrap();
    last_work_session.stop = Some(stop_time);
    if description.is_some() {
        last_work_session.description = desc;
    }
    if homeoffice {
        last_work_session.homeoffice = homeoffice;
    }
    time_sheet.work_sessions.push(last_work_session);
    time_sheet.save(&path).unwrap();
    Ok(())
}

/// Switch from one working session to the next.
pub fn switch_working_sessions(
    description: Option<&str>,
    homeoffice: bool,
) -> Result<(), TimetrackerError> {
    stop_working_session(description, homeoffice)?;
    start_working_session(None, homeoffice)
}

pub fn analyze_work_sheet(_project: Option<&str>) -> Result<(), TimetrackerError> {
    let path = Path::new("time_sheet.json");
    let time_sheet = TimeSheet::load(&path)?;
    let mut work_time: f32 = 0.;
    let mut project_cost: f32 = 0.;

    let mut project_table = Table::new();
    project_table.add_row(row!["Project", time_sheet.project_name]);
    if let Some(r) = time_sheet.hourly_rate {
        project_table.add_row(row!["Hourly Rate", r->format!("{:.02}€", r)]);
    }
    project_table.printstd();

    println!();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let mut homeoffice_map: HashMap<String, Vec<Date<Local>>> = HashMap::new();

    match time_sheet.hourly_rate {
        Some(_) => table.set_titles(row![
            "ID",
            "Start",
            "Stop",
            "HO",
            "Time [h]",
            "Cost [€]",
            "Description"
        ]),
        None => table.set_titles(row!["ID", "Start", "Stop", "HO", "Time [h]", "Description"]),
    }

    for (i, work_session) in time_sheet.work_sessions.iter().enumerate() {
        let split_description = split_description_string(&work_session.description, 44);
        let stop_time = match work_session.stop {
            Some(s) => s,
            None => Local::now(),
        };
        let duration = (stop_time - work_session.start).num_minutes() as f32 / 60f32;
        work_time += duration;
        let homeoffice_mark;
        if work_session.homeoffice {
            homeoffice_mark = "✔";
        } else {
            homeoffice_mark = "";
        }
        match time_sheet.hourly_rate {
            Some(r) => {
                let session_cost = duration * r;
                table.add_row(row![
                    r->i,
                    work_session.start.format(DATETIME_FORMAT),
                    stop_time.format(DATETIME_FORMAT),
                    homeoffice_mark,
                    r->format!("{:.02}", duration),
                    r->format!("{:.02}", session_cost),
                    split_description
                ]);
                project_cost += session_cost;
            }
            None => {
                table.add_row(row![
                    r->i,
                    work_session.start.format(DATETIME_FORMAT),
                    stop_time.format(DATETIME_FORMAT),
                    homeoffice_mark,
                    r->format!("{:.02}h", duration),
                    split_description
                ]);
            }
        };

        let work_date = work_session.start.date();
        let year = format!("{}", work_date.format("%Y"));

        if homeoffice_map.get(&year).is_none() {
            homeoffice_map.insert(year.clone(), Vec::new());
        }

        let mut homeoffice_vec = homeoffice_map.get(&year).unwrap().clone();

        if !homeoffice_vec.contains(&work_date) && work_session.homeoffice {
            homeoffice_vec.push(work_date);
            homeoffice_map.insert(year, homeoffice_vec);
        }
    }

    table.printstd();

    println!();

    let mut homeoffice_table = Table::new();
    homeoffice_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    homeoffice_table.set_titles(row!["year", "days in homeoffice"]);
    let mut year_vec = homeoffice_map.keys().collect::<Vec<&String>>();
    year_vec.sort();
    for year in year_vec {
        homeoffice_table.add_row(row![
            year,
            homeoffice_map.get(year).unwrap_or(&Vec::new()).len()
        ]);
    }
    homeoffice_table.printstd();

    println!();

    let mut total_table = Table::new();
    total_table.add_row(row!["Total work time", r->format!("{:.02}h", work_time)]);
    if time_sheet.hourly_rate.is_some() {
        total_table.add_row(row!["Total project cost", r->format!("{:.02}€", project_cost)]);
    }
    total_table.printstd();
    Ok(())
}

pub fn add_work_session_to_time_sheet(
    _project: Option<&str>,
    start: &str,
    stop: Option<&str>,
    description: Option<&str>,
    homeoffice: bool,
) -> Result<(), TimetrackerError> {
    let work_session = WorkSession::new(
        Local.datetime_from_str(start, DATETIME_FORMAT)?,
        match stop {
            Some(s) => Some(Local.datetime_from_str(s, DATETIME_FORMAT)?),
            None => None,
        },
        match description {
            Some(d) => String::from(d),
            None => String::from(""),
        },
        homeoffice,
    );

    let time_sheet_path = Path::new("time_sheet.json");
    let mut time_sheet = TimeSheet::load(&time_sheet_path)?;
    time_sheet.work_sessions.push(work_session);
    time_sheet.work_sessions.sort();
    time_sheet.save(&time_sheet_path)?;
    Ok(())
}

pub fn add_subproject(name: &str, description: &str) -> Result<(), TimetrackerError> {
    //! Add a new subproject to the time sheet

    println!("{} | {}", name, description);
    let time_sheet_path = Path::new("time_sheet.json");
    let mut time_sheet = TimeSheet::load(&time_sheet_path)?;
    let subproject = SubProject::new(
        time_sheet.subprojects.len(),
        name.to_string(),
        description.to_string(),
    );
    time_sheet.subprojects.push(subproject);
    time_sheet.save(&time_sheet_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /*
    prop_compose! {
        fn compose_work_sessions(max_length: usize)(work_sessions in any_with::<Vec<WorkSession>>(proptest::collection::size_range(max_length).lift())) -> Vec<WorkSession> {
            work_sessions
        }
    }
    */

    proptest! {
        #[test]
        fn test_subproject_creation(id: usize, name in "\\PC*", description in "\\PC*") {
            let subproject = SubProject::new(id, name.clone(), description.clone());
            prop_assert_eq!(subproject.id, id);
            prop_assert_eq!(subproject.name, name);
            prop_assert_eq!(subproject.description, description);
        }

        #[test]
        fn test_subproject_cmp(id_1: usize, id_2: usize, name_1 in "\\PC*", name_2 in "\\PC*", description_1 in "\\PC*", description_2 in "\\PC*") {
            let subproject_1 = SubProject::new(id_1, name_1, description_1);
            let subproject_2 = SubProject::new(id_2, name_2, description_2);
            prop_assert!((id_1 < id_2) == (subproject_1 < subproject_2));
            prop_assert!((id_1 > id_2) == (subproject_1 > subproject_2));
            prop_assert!((id_1 == id_2) == (subproject_1 == subproject_2));
        }

        #[test]
        fn test_time_sheet_creation(project_name in "\\PC*", hourly_rate: f32) {
            let time_sheet = TimeSheet::new(project_name.clone(), Some(hourly_rate));
            prop_assert_eq!(time_sheet.project_name, project_name);
            prop_assert_eq!(time_sheet.hourly_rate, Some(hourly_rate));
            assert_eq!(time_sheet.work_sessions.len(), 0);
        }
    }
}
