use pdf_extract::extract_text;
use std::fs;
use std::path::{Path, PathBuf};

fn get_pdf_files(folder: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let files_in_folder = fs::read_dir(folder)?;
    let pdfs = files_in_folder
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .map(|ext| ext.eq_ignore_ascii_case("pdf"))
                .unwrap_or(false)
        })
        .collect();
    Ok(pdfs)
}

fn parse_and_tokenize(pdf_file: &Path)-> Result<Vec<String>, pdf_extract::OutputError> {
    let pdf_content = extract_text(pdf_file)?;
    let tokens: Vec<String> = pdf_content
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect();
    Ok(tokens)    
}
fn main() {
    let folder =Path::new(".\\data");
    if let Ok(pdf_files) = get_pdf_files(folder) {
        if pdf_files.is_empty() {
            println!("No PDF files found in the specified folder.");
        } else {
            for pdf_file in pdf_files {
                println!("Processing file: {}", pdf_file.display());
                if let Ok(tokens) = parse_and_tokenize(&pdf_file) {
                    println!("Extracted {} tokens from {}", tokens.len(), pdf_file.display());
                    println!("First 20 tokens:");
                    for token in tokens.iter().take(20) {
                        println!("{}", token);
                    }
                } else {
                    println!("Failed to parse or tokenize file: {}", pdf_file.display());
                }
            }
        }
    } else {
        println!("Failed to read the folder: {}", folder.display());
    }    

    // let pdf_file = Path::new("./data/A comparative analysis of currently used microscopic and macroscopic traffic simulation software.pdf");
    // let output_path = Path::new("./data/ExtractedText.txt");

    // if let Ok(pdf_content) = extract_text(pdf_file) {
    //     if let Err(e) = std::fs::write(output_path, &pdf_content) {
    //         println!("Failed to write extracted text to file: {}", e);
    //     } else {
    //         println!("Extracted text saved to {}", output_path.display());
    //     }
    // }
    
}
