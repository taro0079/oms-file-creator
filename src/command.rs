use std::{fs::{File, self}, path::Path, io::Write};

    pub fn writer(domain_name: &str) -> std::io::Result<()> {
    let command_path = Path::new("./Application/Command");
    let comp_command_path = format!("./Application/Command/{}Command.php", domain_name);
    let comp_command_handler_path =
        format!("./Application/Command/{}CommandHandler.php", domain_name);
    let command_file_path = Path::new(&comp_command_path);
    let command_handler_file_path = Path::new(&comp_command_handler_path);

    // ディレクトリの作成
    if command_path.exists() {
        fs::create_dir_all("./Application/Command")?;
    }

    // Commandファイルの作成と内容の書き込み
    if !command_file_path.exists() {
        let mut command_file = File::create(command_file_path)?; // ファイル作成
        let content = format!(
            "<?php\n\ndeclare(strict_types=1);\n\nnamespace App\\{}\\Application\\Command;\n\nuse App\\Shared\\Application\\Command\\CommandInterface;\n\nfinal class {}Command implements CommandInterface\n{{\n}}",
            domain_name, domain_name
        ); // ファイルの内容
        command_file.write_all(content.as_bytes())?; // ファイルに書き込み
    }

    // CommandHandlerファイルの作成と内容の書き込み
    if !command_handler_file_path.exists() {
        let mut command_handler_file = File::create(command_handler_file_path)?;
        let content = format!(
            "<?php\n\ndeclare(strict_types=1);\n\nnamespace App\\{}\\Application\\Command;\n\nuse App\\Shared\\Application\\Command\\CommandHandlerInterface;\n\nfinal class {}CommandHandler implements CommandHandlerInterface\n{{\n}}",
            domain_name, domain_name
        );
        command_handler_file.write_all(content.as_bytes())?;
    }
    Ok(())

    }
