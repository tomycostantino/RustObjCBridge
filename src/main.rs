mod nsworkspace;
use nsworkspace::NSWorkspace;

fn main() {
    run_examples();
}

fn run_examples() {
    let workspace = NSWorkspace::new();

    println!("MacOS NSWorkspace Example");
    println!("------------------------");

    // Get menu bar owning application
    match workspace.get_menu_bar_owning_application() {
        Ok(Some(app)) => {
            println!("Menu bar owning application: {}", app.localized_name);
        },
        Ok(None) => println!("No menu bar owning application found"),
        Err(e) => println!("Error getting menu bar owning application: {}", e),
    }

    // Get frontmost application
    match workspace.get_frontmost_application() {
        Ok(Some(app)) => {
            println!("\nFrontmost Application:");
            println!("Name: {}", app.localized_name);
            println!("Bundle ID: {}", app.bundle_identifier);
            println!("Path: {}", app.executable_path);
            println!("Process ID: {}", app.process_id);
            println!("Launch Date: {}", app.launch_date);
        },
        Ok(None) => println!("\nNo frontmost application found"),
        Err(e) => println!("\nError getting frontmost application: {}", e),
    }

    // Get all running applications
    match workspace.get_running_applications() {
        Ok(apps) => {
            println!("\nRunning Applications:");
            for (i, app) in apps.iter().enumerate() {
                println!("{}. {} ({})", i + 1, app.localized_name, app.bundle_identifier);
            }
        },
        Err(e) => println!("Error getting running applications: {}", e),
    }

    // Open Safari
    let safari_bundle_id = "com.apple.Safari";
    match workspace.launch_application(safari_bundle_id) {
        Ok(true) => println!("\nSuccessfully launched Safari"),
        Ok(false) => println!("\nFailed to launch Safari"),
        Err(e) => println!("\nError launching Safari: {}", e),
    }

    // Get path for an application
    match workspace.get_application_path("com.apple.finder") {
        Ok(Some(path)) => println!("\nFinder path: {}", path),
        Ok(None) => println!("\nCould not find Finder path"),
        Err(e) => println!("\nError getting Finder path: {}", e),
    }

    // Open a URL
    match workspace.open_url("https://www.rust-lang.org") {
        Ok(true) => println!("\nSuccessfully opened Rust website"),
        Ok(false) => println!("\nFailed to open Rust website"),
        Err(e) => println!("\nError opening URL: {}", e),
    }

    // Example 5: Open a file
    // Uncomment and run with a valid file
    /*
    match workspace.open_file("/Users/Shared/test.txt") {
        Ok(true) => println!("\nSuccessfully opened text file"),
        Ok(false) => println!("\nFailed to open text file (might not exist)"),
        Err(e) => println!("\nError opening file: {}", e),
    }
    */
}
