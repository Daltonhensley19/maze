use std::path::{Path, PathBuf};

use crate::error::MazeError;

struct Foo {}

pub fn generate_main_cpp_file<P: AsRef<Path>>(project_name: P) {
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

pub fn generate_cmake_lists<P: AsRef<Path>>(project_name: P) {
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

pub fn generate_gitignore<P: AsRef<Path>>(project_name: P) {
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

pub fn generate_maze_project<P: AsRef<Path>>(project_name: P) -> Result<(), MazeError> {
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
