use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

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

    let description_argument = Arg::with_name("description")
        .value_name("DESCRIPTION")
        .help("A description of what you are going to do");

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
                .arg(&description_argument),
        )
        .subcommand(
            SubCommand::with_name("stop")
                .about("Stop working")
                .author(crate_authors!())
                .version(crate_version!())
                .arg(&project_option)
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
        .get_matches();
    println!("{:#?}", matches);
}
