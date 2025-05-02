// Commands:
// Maze run -- This command is used to run Maze project
// Maze build -- This command is used to build Maze project
// Maze clean -- This command is used to clean Maze project
//

pub mod error;
pub mod generate;

use std::{path::PathBuf, process::Command};

use clap::{Parser, Subcommand};
use error::MazeError;
use generate::{generate_class, generate_maze_project};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run the project")]
    Run,
    #[command(about = "Clean the project")]
    Clean,
    #[command(about = "Build the project")]
    Build,
    #[command(about = "Generate a new project")]
    New { project_name: PathBuf },
    #[command(about = "Generate a new class")]
    CreateClass { class_name: String },
}

fn build_project() -> Result<(), MazeError> {
    // Make sure the cmake file exists
    if !std::fs::exists("CMakeLists.txt").expect("Could not check if CMakeLists.txt exists") {
        return Err(MazeError::CMakeFileNotFound);
    }

    // If build directory does not exist, make one
    if !std::fs::exists("build").expect("Could not check if build directory exists") {
        std::fs::create_dir("build").expect("Could not create build directory");
    }

    // Change directory to build directory
    std::env::set_current_dir("build").expect("Could not change directory");

    // Run cmake command
    let cmake_ret = Command::new("cmake")
        .arg("..")
        .spawn()
        .expect("Could not run cmake command")
        .wait_with_output()
        .expect("Could not run cmake command");

    println!("{}", String::from_utf8_lossy(&cmake_ret.stdout));
    println!("{}", String::from_utf8_lossy(&cmake_ret.stderr));

    // Run make command
    let make_ret = Command::new("make")
        .spawn()
        .expect("Could not run make command")
        .wait_with_output()
        .expect("Could not wait for make command");

    print!("{}", String::from_utf8_lossy(&make_ret.stdout));
    print!("{}", String::from_utf8_lossy(&make_ret.stderr));

    // Change back to original directory
    std::env::set_current_dir("..").expect("Could not change directory");

    Ok(())
}

fn run_project() -> Result<(), MazeError> {
    // Make sure that the project has been build first
    if !std::fs::exists("build").expect("Could not check if build directory exists") {
        build_project()?;
    }

    // Change directory to build directory
    std::env::set_current_dir("build").expect("Could not change directory");

    // Run the project
    let ret = Command::new("./MyProject")
        .spawn()
        .expect("Could not run project")
        .wait_with_output()
        .expect("Could not run project");

    print!("{}", String::from_utf8_lossy(&ret.stdout));
    print!("{}", String::from_utf8_lossy(&ret.stderr));

    // Change back to original directory
    std::env::set_current_dir("..").expect("Could not change directory");

    Ok(())
}

fn clean_project() -> std::io::Result<()> {
    // If build directory exists, remove it
    if std::fs::exists("build")? {
        std::fs::remove_dir_all("build")?;
        println!("Cleaned project (`build/` removed)");
    } else {
        println!("Project already cleaned");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Run => run_project()?,
        Commands::Build => build_project()?,
        Commands::Clean => clean_project()?,
        Commands::CreateClass { class_name } => {
            println!("Creating new class `{:?}`", class_name);
            generate_class(class_name)?;
        }
        Commands::New { project_name } => {
            println!("Creating new project `{:?}`", project_name);
            generate_maze_project(project_name)?;
        }
    }

    Ok(())
}
