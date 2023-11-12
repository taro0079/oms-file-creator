use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

// Resourceファイル
pub fn writer(domain_name: &str) -> std::io::Result<()> {
    let resource_path = Path::new("./Presentation/Api/Resource");
    let comp_resource_path = format!("./Presentation/Api/Resource/{}Resource.php", domain_name);
    let resource_file_path = Path::new(&comp_resource_path);

    if !resource_path.exists() {
        fs::create_dir_all("./Presentation/Api/Resource")?;
    }

    // Resourceファイルの作成と内容の書き込み
    if !resource_file_path.exists() {
        let mut resource_file = File::create(resource_file_path)?; // ファイル作成
        let content_header = format!(
            "<?php\n\ndeclare(strict_types=1);\n\nnamespace App\\{}\\Presentation\\Api\\Resource;",
            domain_name
        );
        let content_class = format!("final class {}Resource implements\n{{\n", domain_name);
        resource_file.write_all(content_header.as_bytes());
        resource_file.write_all(content_class.as_bytes());
    }
    Ok(())
}
