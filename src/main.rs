// Commands:
// Maze run -- This command is used to run Maze project
// Maze build -- This command is used to build Maze project
// Maze clean -- This command is used to clean Maze project
//

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Parser, Subcommand};
use thiserror::Error;

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
}

#[derive(Debug, Error)]
enum MazeError {
    #[error("The project name `{0}` already exists in the current directory")]
    ProjectExists(PathBuf),

    #[error("The CMakeLists.txt file does not exist in the project directory")]
    CMakeFileNotFound,
}

fn generate_main_cpp_file<P: AsRef<Path>>(project_name: P) {
    let main_cpp_path = project_name.as_ref().join("src/main.cpp");
    std::fs::File::create(&main_cpp_path).expect("Could not create main.cpp file");

    let main_cpp: &'static str = r#"#include <iostream>

int main() {
    std::cout << "Hello, World!\n"; 
    return 0;
}
"#;
    std::fs::write(main_cpp_path, main_cpp).expect("Unable to write into main.cpp file");
}

fn generate_cmake_lists<P: AsRef<Path>>(project_name: P) {
    let project_name = project_name.as_ref();
    let cmake_lists_path = project_name.join("CMakeLists.txt");
    std::fs::File::create(&cmake_lists_path).expect("Could not create CMakeLists.txt file");

    let cmake_lists: &'static str = r#"cmake_minimum_required(VERSION 3.10)
project(MyProject)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Glob source files
file(GLOB_RECURSE SOURCES "src/*.cpp" "src/*.cc" "src/*.cxx")
file(GLOB_RECURSE HEADERS "include/*.hpp" "include/*.h" "include/*.hxx")

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE include)

# Optional: Add compiler warnings
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE /W4 /WX)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()

# Generate compile_commands.json (alternative method if needed)
# This is sometimes needed for older CMake versions
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE INTERNAL "")

"#;

    std::fs::write(cmake_lists_path, cmake_lists)
        .expect("Unable to write into CMakeLists.txt file");
}

fn generate_gitignore<P: AsRef<Path>>(project_name: P) {
    let gitignore_path = project_name.as_ref().join(".gitignore");
    std::fs::File::create(&gitignore_path).expect("Could not create .gitignore file");

    let gitignore: &'static str = r#"# Compiled Object files
*.o
*.ko
*.obj
*.elf

# Precompiled Headers
*.gch
*.pch

# Compiled Dynamic libraries
*.so
*.dylib
*.dll

# Executables
*.exe
*.out
*.app

# CMake build directory
build/
[Bb]uild*/
[Cc]make*/
[Oo]utput/

# CMake generated files
CMakeCache.txt
CMakeFiles/
cmake_install.cmake
install_manifest.txt
compile_commands.json
*.cmake
*.user
Makefile

# IDE specific files
.vscode/
.idea/
*.swp
*.swo
*.sublime-workspace
*.sublime-project

# Dependency directories
deps/
[Dd]ependencies/

# Test binaries
[Tt]ests/
[Tt]est/
*.test

# macOS
.DS_Store

# Visual Studio
*.vcxproj*
*.sln
*.suo
*.opensdf
*.sdf
*.VC.db
*.VC.VC.opendb

# Qt Creator
*.autosave

# CLion
cmake-build-*/

# Coverage
*.gcda
*.gcno
*.gcov

# Documentation
[Dd]ocs/
[Dd]oxygen/

# Binaries for program debuggers
*.dSYM/
*.stackdump

# Temporary files
*~
*.tmp
*.bak
*.swp
*.swo
"#;

    std::fs::write(gitignore_path, gitignore).expect("Unable to write into .gitignore file");
}

fn generate_maze_project<P: AsRef<Path>>(project_name: P) -> Result<(), MazeError> {
    // Make sure that the project_name does not already exist in the current directory
    let project_name = project_name.as_ref();
    if std::fs::exists(project_name).expect("Could not check if file exists") {
        return Err(MazeError::ProjectExists(PathBuf::from(project_name)));
    }

    // Generate project directory
    std::fs::create_dir(project_name).expect("Could not create project directory");

    // Generate project src directory
    std::fs::create_dir(project_name.join("src/")).expect("Could not create src directory");

    // Generate project include directory
    std::fs::create_dir(project_name.join("include/")).expect("Could not create include directory");

    // Generate project main.cpp and populate with minimal stub
    generate_main_cpp_file(project_name);

    // Generate project CMakeLists.txt and populate with minimal stub
    generate_cmake_lists(project_name);

    // Generate project .gitignore and populate with minimal stub
    generate_gitignore(project_name);

    Ok(())
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
        Commands::New { project_name } => {
            println!("Creating new project `{:?}`", project_name);
            generate_maze_project(project_name)?;
        }
    }

    Ok(())
}
