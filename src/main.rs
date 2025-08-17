use pdf_extract::extract_text;
use std::fs;
use std::path::{Path, PathBuf};
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;
use rayon::prelude::*;

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
    let stemmer = Stemmer::create(Algorithm::English);
    let tokens: Vec<String> = pdf_content
        .split(|c: char| !c.is_alphanumeric() && c != '-') // keep hyphens inside tokens
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .flat_map(|token| {
            if token.contains('-') {
                let joined = token.replace('-', "");
                let separated = token.replace('-', " ");
                vec![token.clone(), joined, separated]
            } else {
                vec![token]
            }
        })
        .filter(|s| !s.is_empty())
        .map(|s| stemmer.stem(&s).to_string())
        .collect();
    Ok(tokens)  
}


// Tokenize a single PDF and return counts
fn unique_tokens_count(pdf_file: &Path) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    if let Ok(tokens) = parse_and_tokenize(pdf_file) {
        for token in tokens {
            *counts.entry(token).or_insert(0) += 1;
        }
    }
    counts
}
fn main() {
    let folder =Path::new(".\\data");
    if let Ok(pdf_files) = get_pdf_files(folder) {
        if pdf_files.is_empty() {
            println!("No PDF files found in the specified folder.");
        } else {

            // Stage 1: Parallel tokenization
            let pdf_token_counts: Vec<(String, HashMap<String, usize>)> = pdf_files
                .par_iter()
                .map(|pdf_file| (pdf_file.display().to_string(), unique_tokens_count(pdf_file)))
                .collect();

            // Stage 2: Merge into global TF
            let mut tf: HashMap<String, HashMap<String, usize>> = HashMap::new();

            for (pdf_name, counts) in pdf_token_counts {
                for (token, count) in counts {
                    let doc_counts = tf.entry(token).or_default();
                    doc_counts.insert(pdf_name.clone(), count);
                }
            }

            // Debug: print TF for first few tokens
                for (token, docs) in tf.iter().take(5) {
                    println!("Token: {}", token);
                    for (doc, count) in docs {
                        println!("  {} -> {}", doc, count);
                    }
                }

        }
    } else {
        println!("Failed to read the folder: {}", folder.display());
    }    
}
