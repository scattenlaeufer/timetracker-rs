// use chrono::prelude::*;
use chrono::prelude::*;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::path::Path;

fn main() {
    let rate_option = Arg::with_name("rate")
        .short("r")
        .long("rate")
        .value_name("RATE")
        .validator(|s: String| match &s.parse::<f32>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from(
                "Must be convertible to floating point number!",
            )),
        })
        .help("Hourly rate");

    let project_argument = Arg::with_name("project")
        .value_name("PROJECT")
        .help("The project to analyze");

    let project_option = Arg::with_name("project")
        .short("p")
        .long("project")
        .value_name("PROJECT")
        .help("Project to stop work on");

    fn time_validator(s: String) -> Result<(), String> {
        match Local.datetime_from_str(&s, timetracker::DATETIME_FORMAT) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Must comply with \"{}\" format!",
                timetracker::DATETIME_FORMAT
            )),
        }
    }

    let start_help_string = format!(
        "Start time of the work session, formatted as \"{}\"",
        timetracker::DATETIME_FORMAT
    );

    let start_option = Arg::with_name("start")
        .short("b")
        .long("start")
        .value_name("START-TIME")
        .validator(time_validator)
        .help(&start_help_string);

    let stop_help_string = format!(
        "Stop time of the work session, formatted as \"{}\"",
        timetracker::DATETIME_FORMAT
    );
    let stop_option = Arg::with_name("stop")
        .short("e")
        .long("stop")
        .value_name("STOP-TIME")
        .validator(time_validator)
        .help(&stop_help_string);

    let description_option = Arg::with_name("description")
        .short("d")
        .long("description")
        .value_name("DESCRIPTION")
        .help("A description of what was done during this work session");

    let work_session_id_option = Arg::with_name("work_session_id")
        .short("i")
        .long("id")
        .value_name("ID")
        .required(true)
        .validator(|s: String| match &s.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Must be a unsigned integer!")),
        })
        .help("Id of the work session to be edited");

    let description_argument = Arg::with_name("description")
        .value_name("DESCRIPTION")
        .help("A description of what you did");

    let homeoffice_option = Arg::with_name("homeoffice")
        .short("h")
        .long("homeoffice")
        .help("Track whether a day was spend in homeoffice or not");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new project")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .value_name("NAME")
                        .help("Name of the project"),
                )
                .arg(&rate_option),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("Start working")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&project_option)
                .arg(&homeoffice_option)
                .arg(&description_argument),
        )
        .subcommand(
            SubCommand::with_name("stop")
                .about("Stop working")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&project_option)
                .arg(&homeoffice_option)
                .arg(&description_argument),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Change settings for a given project")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&project_argument)
                .arg(&rate_option),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze all tracked time for a given project")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&project_argument),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List all projects")
                .author(crate_authors!())
                .version(crate_version!()),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a work session to a given project")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(start_option.clone().required(true))
                .arg(&stop_option)
                .arg(&description_option)
                .arg(&homeoffice_option)
                .arg(&project_argument),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit a work session to a given project")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&work_session_id_option)
                .arg(&start_option)
                .arg(&stop_option)
                .arg(&description_option)
                .arg(&project_argument),
        )
        .subcommand(
            SubCommand::with_name("switch")
                .about("Switch from one work session to the next")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&project_option)
                .arg(&description_argument),
        )
        .subcommand(
            SubCommand::with_name("activities")
                .about("Manage separate activities with a project")
                .author(crate_authors!())
                .version(crate_version!())
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a new activity")
                        .author(crate_authors!())
                        .version(crate_version!()),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove a given activity")
                        .author(crate_authors!())
                        .version(crate_version!()),
                )
                .subcommand(
                    SubCommand::with_name("edit")
                        .about("Edit a given activity")
                        .author(crate_authors!())
                        .version(crate_version!()),
                )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List all activities")
                        .author(crate_authors!())
                        .version(crate_version!()),
                ),
        )
        .subcommand(
            SubCommand::with_name("subprojects")
                .about("Manage subprojects within project")
                .author(crate_authors!())
                .version(crate_version!())
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a new subproject")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .arg(
                            Arg::with_name("name")
                                .short("n")
                                .long("name")
                                .value_name("NAME")
                                .required(true)
                                .help("A name identifier for a new subproject"),
                        )
                        .arg(
                            Arg::with_name("description")
                                .short("d")
                                .long("description")
                                .value_name("DESCRIPTION")
                                .required(true)
                                .help("A description for a new subproject"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove a given subproject")
                        .author(crate_authors!())
                        .version(crate_version!()),
                )
                .subcommand(
                    SubCommand::with_name("edit")
                        .about("Edit a given subproject")
                        .author(crate_authors!())
                        .version(crate_version!()),
                )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List all subprojects")
                        .author(crate_authors!())
                        .version(crate_version!()),
                )
                .subcommand(
                    SubCommand::with_name("export")
                        .about("Export a given subproject")
                        .author(crate_authors!())
                        .version(crate_version!()),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        println!("{:#?}", matches);
        let rate = match matches.value_of("rate") {
            Some(r) => Some(r.parse::<f32>().unwrap()),
            None => None,
        };
        let path = Path::new("time_sheet.json");
        timetracker::initialize_project(matches.value_of("name").unwrap().to_string(), rate, &path)
            .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("start") {
        timetracker::start_working_session(
            matches.value_of("description"),
            matches.occurrences_of("homeoffice") > 0,
        )
        .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("stop") {
        timetracker::stop_working_session(
            matches.value_of("description"),
            matches.occurrences_of("homeoffice") > 0,
        )
        .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("analyze") {
        timetracker::analyze_work_sheet(matches.value_of("project")).unwrap();
    }

    if let Some(_matches) = matches.subcommand_matches("list") {
        println!("Subcommand list is not implemented yet.")
    }

    if let Some(_matches) = matches.subcommand_matches("config") {
        println!("Subcommand config is not implemented yet.")
    }

    if let Some(subcommand_matches) = matches.subcommand_matches("switch") {
        timetracker::switch_working_sessions(
            subcommand_matches.value_of("description"),
            matches.occurrences_of("homeoffice") > 0,
        )
        .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        timetracker::add_work_session_to_time_sheet(
            matches.value_of("project"),
            matches.value_of("start").unwrap(),
            matches.value_of("stop"),
            matches.value_of("description"),
            matches.occurrences_of("homeoffice") > 0,
        )
        .unwrap();
    }

    if let Some(_matches) = matches.subcommand_matches("edit") {
        println!("Subcommand edit is not implemented yet.")
    }

    if let Some(matches) = matches.subcommand_matches("activities") {
        if let Some(_matches) = matches.subcommand_matches("add") {
            println!("Subcommand add is not implemented yet.")
        }
        if let Some(_matches) = matches.subcommand_matches("remove") {
            println!("Subcommand remove is not implemented yet.")
        }
        if let Some(_matches) = matches.subcommand_matches("edit") {
            println!("Subcommand edit is not implemented yet.")
        }
        if let Some(_matches) = matches.subcommand_matches("list") {
            println!("Subcommand list is not implemented yet.")
        }
    }

    if let Some(matches) = matches.subcommand_matches("subprojects") {
        if let Some(matches) = matches.subcommand_matches("add") {
            timetracker::add_subproject(
                matches.value_of("name").expect("No name given!"),
                matches
                    .value_of("description")
                    .expect("no description given!"),
            )
            .unwrap();
        }
        if let Some(_matches) = matches.subcommand_matches("remove") {
            println!("Subcommand remove is not implemented yet.")
        }
        if let Some(_matches) = matches.subcommand_matches("edit") {
            println!("Subcommand edit is not implemented yet.")
        }
        if let Some(_matches) = matches.subcommand_matches("list") {
            println!("Subcommand list is not implemented yet.")
        }
        if let Some(_matches) = matches.subcommand_matches("export") {
            println!("Subcommand export is not implemented yet.")
        }
    }
}
