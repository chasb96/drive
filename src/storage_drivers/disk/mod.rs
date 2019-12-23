use super::StorageDriver;
use crate::storage_drivers::error::StorageError;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct Disk;

impl Disk {
    pub fn new() -> Self {
        Self
    }
}

impl StorageDriver for Disk {
    fn store(&self, path: &Path, mut contents: File) -> Result<(), StorageError> {
        // Attempt to create the file, without any contents in it
        let mut file = match File::create(Path::new(&path)) {
            Ok(file) => file,
            Err(_) => {
                // Its possible the directory doesn't exist yet
                if let Err(e) = create_dir_all(format!("{}", path.parent().unwrap().display())) {
                    log!("error", "Failed to create directory: {}", e);

                    return Err(StorageError::from(e));
                }

                // Try again, with the new directory, if this fails, give up
                match File::create(Path::new(&path)) {
                    Ok(file) => file,
                    Err(e) => {
                        log!(
                            "error",
                            "Failed to create file {}: {}",
                            path.to_str().unwrap_or(""),
                            e
                        );

                        return Err(StorageError::from(e));
                    }
                }
            }
        };

        // Create a buffer to read and write the contents to
        let mut buffer = [0; 1_000_000];

        // Continually write the contents to the file until there's nothing
        //  left to read
        loop {
            // Read from the input and dump it into the buffer
            let bytes = match contents.read(&mut buffer) {
                Ok(bytes) => bytes,
                Err(e) => {
                    log!("error", "Failed to read buffer: {}", e);

                    return Err(StorageError::from(e));
                }
            };

            // If nothing was read, all is done
            if bytes == 0 {
                break;
            }

            // Dump the buffer into the new file
            if let Err(e) = file.write(&buffer[..bytes]) {
                log!("error", "Failed to write buffer: {}", e);

                return Err(StorageError::from(e));
            }
        }

        // Flush out the last of the writing
        if let Err(e) = file.flush() {
            log!("error", "Failed to flush buffer: {}", e);

            return Err(StorageError::from(e));
        }

        Ok(())
    }

    fn read(&self, path: &Path) -> Result<File, StorageError> {
        match File::open(path) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!(
                    "error",
                    "Failed to open file {}: {}",
                    path.to_str().unwrap_or(""),
                    e
                );

                Err(StorageError::from(e))
            }
        }
    }

    fn delete(&self, path: &Path) -> Result<(), StorageError> {
        match remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) => {
                log!(
                    "error",
                    "Failed to delete file {}: {}",
                    path.to_str().unwrap_or(""),
                    e
                );

                Err(StorageError::from(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_store() -> Result<(), Box<dyn Error>> {
    //     let path = Path::new("storage/test/store");
    //     let expected = vec![10, 10, 10, 10, 10];
    //     let mut actual = Vec::new();
    //
    //     // Disk::store(path, &mut expected.as_slice())?;
    //
    //     let mut file = File::open(path)?;
    //     file.read_to_end(&mut actual)?;
    //
    //     assert_eq!(expected, actual);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_read() -> Result<(), Box<dyn Error>> {
    //     let path = Path::new("storage/test/read");
    //
    //     {
    //         File::create(path)?;
    //     }
    //
    //     Disk::read(path)?;
    //
    //     Ok(())
    // }
}
