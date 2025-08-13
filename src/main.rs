use pdf_extract::extract_text;

// use std::collections::HashMap;
// use std::path::Path;

fn main() {
    let file = "./data/TextTwo.txt";
    let pdf_file = "./data/Integrated_Lane_Changing_Models.pdf";
    // match extract_text(pdf_file) {
    //     Ok(text) => println!("PDF Content:\n{}", text),
    //     Err(e) => println!("Failed to extract PDF content: {}", e),
    // }
    if let Ok(text) = extract_text(pdf_file) {
        let output_path = "./data/ExtractedText.txt";
        if let Err(e) = std::fs::write(output_path, &text) {
            println!("Failed to write extracted text to file: {}", e);
        } else {
            println!("Extracted text saved to {}", output_path);
        }
    }
    // Example of reading a file
    if let Ok(content) = std::fs::read_to_string(file)
    {
        println!("Content of {}: {}", file, content);
    } else {
        println!("Failed to read the file: {}", file);
    } 
    
    
}
