mod nsworkspace;
use nsworkspace::NSWorkspace;

fn main() {
    let workspace = NSWorkspace::new();

    println!("MacOS NSWorkspace Example");
    println!("------------------------");

    // Example 1: Get all running applications
    match workspace.get_running_applications() {
        Ok(apps) => {
            println!("\nRunning Applications:");
            for (i, app) in apps.iter().enumerate() {
                println!("{}. {} ({})", i + 1, app.localized_name, app.bundle_identifier);
            }
        },
        Err(e) => println!("Error getting running applications: {}", e),
    }

    // Example 2: Open Safari
    let safari_bundle_id = "com.apple.Safari";
    match workspace.launch_application(safari_bundle_id) {
        Ok(true) => println!("\nSuccessfully launched Safari"),
        Ok(false) => println!("\nFailed to launch Safari"),
        Err(e) => println!("\nError launching Safari: {}", e),
    }

    // Example 3: Get path for an application
    match workspace.get_application_path("com.apple.finder") {
        Ok(Some(path)) => println!("\nFinder path: {}", path),
        Ok(None) => println!("\nCould not find Finder path"),
        Err(e) => println!("\nError getting Finder path: {}", e),
    }

    // Example 4: Open a URL
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