use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

struct Editor {
    lines: Vec<String>,
}

impl Editor {
    fn new() -> Self {
        Editor { lines: Vec::new() }
    }

    fn load_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        self.lines = reader.lines().collect::<Result<_, _>>()?;
        Ok(())
    }

    fn display(&self) {
        for (index, line) in self.lines.iter().enumerate() {
            println!("{}: {}", index + 1, line);
        }
    }

    fn append(&mut self, line: String) {
        self.lines.push(line);
    }

    fn delete(&mut self, line_number: usize) {
        if line_number == 0 || line_number > self.lines.len() {
            println!("Invalid line number");
        } else {
            self.lines.remove(line_number - 1);
        }
    }

    fn save_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for line in &self.lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

fn main() {
    let mut editor = Editor::new();
    let mut input = String::new();

    println!("RED, Type 'help' for commands.");

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "quit" {
            break;
        } else if input == "help" {
            println!("Commands:");
            println!("load <filename> - Load a file");
            println!("display - Display the file content");
            println!("append <line> - Append a line");
            println!("delete <line_number> - Delete a line");
            println!("save <filename> - Save the file");
            println!("quit - Quit the editor");
        } else if input.starts_with("load ") {
            let filename = &input[5..];
            match editor.load_file(filename) {
                Ok(()) => println!("File loaded successfully"),
                Err(err) => println!("Error loading file: {}", err),
            }
        } else if input == "display" {
            editor.display();
        } else if input.starts_with("append ") {
            let line = &input[7..];
            editor.append(line.to_string());
        } else if input.starts_with("delete ") {
            if let Ok(line_number) = input[7..].parse::<usize>() {
                editor.delete(line_number);
            } else {
                println!("Invalid line number");
            }
        } else if input.starts_with("save ") {
            let filename = &input[5..];
            match editor.save_file(filename) {
                Ok(()) => println!("File saved successfully"),
                Err(err) => println!("Error saving file: {}", err),
            }
        } else {
            println!("Unknown command. Type 'help' for a list of commands.");
        }
    }
}
