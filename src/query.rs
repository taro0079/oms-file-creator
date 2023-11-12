use std::{path::Path, fs::{File, self}, io::Write};

pub fn writer(domain_name: &str) -> std::io::Result<()>{

    let query_path = Path::new("./Application/Query");
    let comp_query_path = format!("./Application/Query/{}Query.php", domain_name);
    let comp_query_handler_path = format!("./Application/Query/{}QueryHandler.php", domain_name);
    let query_file_path = Path::new(&comp_query_path);
    let query_handler_file_path = Path::new(&comp_query_handler_path);

    if !query_path.exists() {
        fs::create_dir_all("./Application/Query")?;
    }
    // Queryファイルの作成と内容の書き込み
    if !query_file_path.exists() {
        let mut query_file = File::create(query_file_path)?; // ファイル作成
        let content = format!(
            "<?php\n\ndeclare(strict_types=1);\n\nnamespace App\\{}\\Application\\Query;\n\n use App\\Shared\\Application\\Query\\QueryInterface;\n\nfinal readonly class {}Query implements QueryInterface\n{{\n}}",
            domain_name, domain_name
        ); // ファイルの内容
        query_file.write_all(content.as_bytes())?; // ファイルに書き込み
    }

    // QueryHandlerファイルの作成と内容の書き込み
    if !query_handler_file_path.exists() {
        let mut query_hander_file = File::create(query_handler_file_path)?;
        let content = format!(
            "<?php\n\ndeclare(strict_types=1);\n\nnamespace App\\{}\\Application\\Query;\n\n use App\\Shared\\Application\\Query\\QueryHandlerInterface;\n\nfinal class {}QueryHandler implements QueryHandlerInterface\n{{\n}}",
            domain_name, domain_name
        );
        query_hander_file.write_all(content.as_bytes())?;
    }
    Ok(())
}
