pub mod unit_tests;

use std::fs;
use std::io::Write;
use std::error::Error;
use clipboard::{ClipboardContext, ClipboardProvider, osx_clipboard::OSXClipboardContext};

const BOOKMARKS_PATH: &str = ".bookmarks";

fn main() -> Result<(), Box<dyn Error>> {
    let content = match fs::read_to_string(BOOKMARKS_PATH) {
        Ok(contents) => contents,
        Err(err) => panic!("Error reading file - {}", err)
    };

    let bookmarks: Vec<&str> = content.lines().collect();
    let mut clipboard: OSXClipboardContext = ClipboardContext::new().unwrap();

    display_list(&bookmarks);

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();

    // Check if user input is a number
    if let Ok(selection) = user_input.trim().parse::<usize>() {
        let item = match bookmarks.get(selection-1) {
            Some(val) => val,
            None => panic!("Error selecting item - item may not exist")
        };

        clipboard.set_contents(item.to_string()).unwrap();

        return Ok(())
    }

    match user_input.trim().to_lowercase().as_str() {
        // Add
        "a" => {
            let clipboard_content = match clipboard.get_contents() {
                Ok(contents) => format!("{}\n", contents),
                Err(err) => panic!("Error: unable to get contents from clipboard - {}", err)
            };

            if let Err(err) = append_to_file(clipboard_content, BOOKMARKS_PATH) {
                panic!("{}", err);
            }
        }
        // Delete
        "d" => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if let Ok(selection) = input.trim().parse::<usize>() {
                if let Err(err) = delete_line_from_file(selection, BOOKMARKS_PATH) {
                    panic!("{}", err);
                }
            }
        }
        // Quit
        "q" => {}
        _ => panic!("Error - invalid option")
    }

    Ok(())
}

/// Appends the contents of `content` to a file.
///
/// # Examples
///
/// ```
/// use clipboard::ClipboardContext;
///
/// const FILE_PATH = "input.txt";
/// let mut clipboard = ClipboardContext::new().unwrap();
///
/// let content = match clipboard.get_contents() {
///     Ok(contents) => format!("{}\n", contents),
///     Err(err) => panic!("Error: unable to get contents from clipboard - {}", err)
/// };
///
/// match add_item(content, FILE_PATH) {
///     Ok(_) => {},
///     Err(err) => panic!("{}", err)
/// }
/// ```
pub fn append_to_file(content: String, file_path: &str) -> Result<(), String> {
    let mut file = match fs::OpenOptions::new()
        .append(true)
        .open(file_path) {
            Ok(contents) => contents,
            Err(err) => return Err(format!("Error reading file - {}", err))
        };

    match file.write_all(content.as_bytes()) {
        Ok(_) => {},
        Err(err) => return Err(format!("Error writing to file - {}", err))
    };

    file.flush().unwrap();

    Ok(())
}

/// Converts each line in a file to a vector, then removes a line specified by `item_index`
///
/// # Examples
///
/// ```
/// const FILE_PATH = "input.txt";
/// let selection = 3;
///
/// match delete_line_from_file(selection, FILE_PATH) {
///     Ok(_) => {},
///     Err(err) => panic!("{}", err)
/// }
/// ```
pub fn delete_line_from_file(item_index: usize, file_path: &str) -> Result<(), String> {
    let content: String = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(format!("Error reading file - {}", err))
    };

    let mut items: Vec<&str> = content.lines().collect();

    items.remove(item_index - 1);
    let contents = items.join("\n");
    println!("{}", contents);

    if let Err(err) = fs::write(file_path, format!("{}\n", contents)) {
        return Err(format!("Error writing to file - {}", err));
    };

    Ok(())
}

/// Prints the contents of a vector to the stdout in order
fn display_list(bookmarks: &Vec<&str>) {
    for index in 0..bookmarks.len() {
        let item = bookmarks[index];

        println!("{} - {}", index+1, item);
    }
}
