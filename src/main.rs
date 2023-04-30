use clap::arg;
use log::*;
use simplelog::*;

mod codegen;
mod parser;

#[derive(Debug)]
struct Args {
    project_path: String,
    debug: bool,
    show_info: bool,
}

fn get_args() -> Args {
    let matches = clap::Command::new("scratchnative")
        .arg(arg!(-d --debug "Enable debugging"))
        .arg(arg!(-i --info "Show info about the project"))
        .arg(arg!([project_path] "Project path").required(true))
        .get_matches();

    let project_path = matches
        .get_one::<String>("project_path")
        .unwrap()
        .to_string();

    Args {
        debug: matches.get_flag("debug"),
        project_path,
        show_info: matches.get_flag("info"),
    }
}

fn exit_with_error<S: Into<String>>(msg: S, error: String) -> ! {
    error!("{}: {}", msg.into(), error);
    std::process::exit(1)
}

fn main() {
    let args = get_args();

    let level_filter = match args.debug {
        false => LevelFilter::Error,
        true => LevelFilter::Trace,
    };

    TermLogger::init(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();

    debug!("program arguments: {:?}", args);

    let path = args.project_path;

    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => exit_with_error("Cannot read file", err.to_string()),
    };

    let scratch_file = parser::parse_scratch_file(content);

    if args.show_info {
        parser::show_info(&scratch_file);
    }

    let proj = parser::scratch_file_to_project(&scratch_file);

    debug!("AST:\n {:#?}", proj);

    let str = codegen::codegen_project(proj);

    println!("{}", str);

    // codegen::codegen_file(scratch_file);
}
