use std::{
    env,
    fs::{self, File},
    path::Path,
};

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let mut cd = String::new();

    if let Some(dir_name) = current_dir.file_name() {
        println!("dir_name is {}", dir_name.to_string_lossy());
        cd = dir_name.to_string_lossy().to_string();
    }
    println!("path is {}", cd);
    let command_path = Path::new("./Application/Command");
    if !command_path.exists() {
        fs::create_dir_all("./Application/Command")?;
    }

    let query_path = Path::new("./Application/Query");
    let comp_query_path = format!("./Application/Query/{}Query.php", cd);
    let comp_query_handler_path = format!("./Application/Query/{}QueryHandler.php", cd);
    let query_file_path = Path::new(&comp_query_path);
    let query_handler_file_path = Path::new(&comp_query_handler_path);

    if !query_path.exists() {
        fs::create_dir_all("./Application/Query")?;
    }
    if !query_file_path.exists() {
        File::create(query_file_path)?;
    }

    if !query_handler_file_path.exists() {
        File::create(query_handler_file_path)?;
    }

    let resource_path = Path::new("./Presentation/Api/Resource");

    if !resource_path.exists() {
        fs::create_dir_all("./Presentation/Api/Resource")?;
    }

    // File::create("./test/test.txt")?;
    Ok(())
}
