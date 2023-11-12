use std::{path::Path, fs::{self, File}, io::Write};

pub fn writer(domain_name: &str) -> std::io::Result<()> {
    // Processorファイル
    let processor_path = Path::new("./Presentation/Api/State/Processor");
    let comp_processor_path = format!(
        "./Presentation/Api/State/Processor/{}Processor.php",
        domain_name
    );
    let processor_file_path = Path::new(&comp_processor_path);

    // ディレクトリの作成
    if !processor_path.exists() {
        fs::create_dir_all("./Presentation/Api/State/Processor")?;
    }

    // Processorファイルの作成と内容の書き込み
    if !processor_file_path.exists() {
        let mut processor_file = File::create(processor_file_path)?; // ファイル作成
        let content = format!(
            "<?php\n\ndeclare(strict_types=1);\n\nnamespace App\\{}\\Presentation\\Api\\State\\Processor;\n\n",
            domain_name
        ); // ファイルの内容
        let content_use = format!("use ApiPlatform\\Metadata\\Operation;\nuse ApiPlatform\\State\\ProcessorInterface;\n\n");
        let content_class = format!("final class {}Processor implements ProcessorInterface\n{{\n", domain_name);
        let content_process = format!("public function process(mixed $data, Operation $operation, array $uriVariables = [], array $context = []): {}Resource\n{{\n}}\n}}", domain_name);

        processor_file.write_all(content.as_bytes())?; // ファイルに書き込み
        processor_file.write_all(content_use.as_bytes())?; // use statement
        processor_file.write_all(content_class.as_bytes())?; // class
        processor_file.write_all(content_process.as_bytes())?; // process method
    }
    Ok(())

}
