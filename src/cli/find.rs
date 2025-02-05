use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn handle(excludes: Vec<String>) -> io::Result<()> {
    // Caminho do arquivo .workpath
    let workpath_file = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".workpath");

    // Ler o conteúdo do arquivo .workpath
    let content = fs::read_to_string(&workpath_file)?;

    // Separar diretórios e arquivos
    let mut directories = Vec::new();
    let mut files = Vec::new();

    for line in content.lines() {
        if line.ends_with('/') {
            directories.push(line.to_string());
        } else {
            files.push(line.to_string());
        }
    }

    // Separar exclusões de diretórios e arquivos
    let mut exclude_dirs = Vec::new();
    let mut exclude_files = Vec::new();

    for exclude in excludes {
        // Dividir a string de exclusões em partes separadas por espaços em branco
        for part in exclude.split_whitespace() {
            if part.ends_with('/') {
                exclude_dirs.push(part.to_string());
            } else {
                exclude_files.push(part.to_string());
            }
        }
    }

    //println!("Exclude dirs: {:?}", exclude_dirs);
    //println!("Exclude files: {:?}", exclude_files);

    // Verificar se os diretórios de exclusão estão presentes no .workpath
    let mut filtered_directories = directories.clone();
    for exclude_dir in &exclude_dirs {
        if directories.contains(exclude_dir) {
            // Remover o diretório da lista de diretórios a serem buscados
            filtered_directories.retain(|dir| dir != exclude_dir);
        }
    }

    //println!("Filtered directories: {:?}", filtered_directories);

    // Verificar se há exclusões de subdiretórios ou arquivos dentro dos diretórios listados
    let mut fd_excludes = Vec::new();
    for exclude in &exclude_dirs {
        for dir in &directories {
            if exclude.starts_with(dir) && exclude != dir {
                // Calcular o caminho relativo
                let relative_path = exclude.replace(dir, "");
                fd_excludes.push(relative_path);
            }
        }
    }

    //println!("Fd Excludes: {:?}", fd_excludes);

    // Executar `fd` para listar arquivos nos diretórios, aplicando exclusões de diretórios e caminhos relativos
    let mut fd_command = Command::new("fd");
    fd_command.arg("-H").arg(".").args(&filtered_directories);

    for exclude in &fd_excludes {
        fd_command.arg("--exclude").arg(exclude);
    }

    //println!("Executando fd: {:?}", fd_command);

    let fd_output = fd_command.output()?;

    // Executar `readlink` para resolver caminhos simbólicos dos arquivos, excluindo arquivos na lista de exclusões
    let mut readlink_output = Vec::new();
    for file in files {
        // Verificar se o arquivo está na lista de exclusões
        if !exclude_files.iter().any(|exclude| {
            let exclude_path = Path::new(exclude);
            let file_path = Path::new(&file);
            file_path == exclude_path
        }) {
            let output = Command::new("readlink").arg("-e").arg(&file).output()?;
            if output.status.success() {
                readlink_output.push(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
    }

    // Combinar saídas de `fd` e `readlink`
    let mut combined_output = String::from_utf8_lossy(&fd_output.stdout).to_string();
    combined_output.push_str(&readlink_output.join("\n"));

    // Passar a saída combinada para o `fzf`
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let fzf_stdin = fzf.stdin.as_mut().expect("Failed to open stdin for fzf");
        fzf_stdin.write_all(combined_output.as_bytes())?;
    }

    // Capturar a saída do `fzf`
    let fzf_output = fzf.wait_with_output()?;

    if fzf_output.status.success() {
        let selected = String::from_utf8_lossy(&fzf_output.stdout)
            .trim()
            .to_string();
        if !selected.is_empty() {
            println!("{}", selected);
        }
    }

    Ok(())
}
