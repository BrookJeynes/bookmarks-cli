use std::fs::{self, File};
use std::io::Write;
use serial_test::serial;

#[cfg(test)]
mod tests {
    use crate::{append_to_file, delete_line_from_file};

    use super::*;

    const TEST_FILE: &str = ".bookmarks-test";

    #[test]
    #[serial]
    fn test_new_item() {
        // Assign
        let mut test_file = File::create(TEST_FILE).unwrap();
        writeln!(&mut test_file, "Test item 1\nTest item 2").unwrap();

        let expected = String::from("Test item 1\nTest item 2\nTest item 3");

        // Act
        let new_item = String::from("Test item 3");
        append_to_file(new_item, TEST_FILE).unwrap();

        let result = fs::read_to_string(TEST_FILE).unwrap();

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    #[serial]
    fn test_delete_item() {
        // Assign
        let mut test_file = File::create(TEST_FILE).unwrap();
        writeln!(&mut test_file, "Test item 1\nTest item 2\nTest item 3").unwrap();

        let expected = String::from("Test item 1\nTest item 3\n");

        // Act
        let item_to_remove = 2;
        delete_line_from_file(item_to_remove, TEST_FILE).unwrap();

        let result = fs::read_to_string(TEST_FILE).unwrap();

        // Assert
        assert_eq!(expected, result);
    }
}
