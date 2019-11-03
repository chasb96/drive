use super::StorageDriver;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct Disk;

impl StorageDriver for Disk {
    type Error = Error;

    fn store(path: &Path, mut contents: File) -> Result<(), Self::Error> {
        // Attempt to create the file, without any contents in it
        let mut file = match File::create(Path::new(&path)) {
            Ok(file) => file,
            Err(_) => {
                // Its possible the directory doesn't exist yet
                if let Err(e) = create_dir_all(format!("{}", path.parent().unwrap().display())) {
                    log!("error", "Failed to create directory: {}", e);
                    return Err(e);
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

                        return Err(e);
                    }
                }
            }
        };

        // Create a buffer to read and write the contents to
        let mut buffer = [0; 1000000];

        // Continually write the contents to the file until there's nothing
        //  left to read
        loop {
            // Read from the input and dump it into the buffer
            let bytes = match contents.read(&mut buffer) {
                Ok(bytes) => bytes,
                Err(e) => {
                    log!("error", "Failed to read buffer: {}", e);
                    return Err(e);
                }
            };

            // If nothing was read, all is done
            if bytes <= 0 {
                break;
            }

            // Dump the buffer into the new file
            if let Err(e) = file.write(&buffer[..bytes]) {
                log!("error", "Failed to write buffer: {}", e);
                return Err(e);
            }
        }

        // Flush out the last of the writing
        if let Err(e) = file.flush() {
            log!("error", "Failed to flush buffer: {}", e);
            return Err(e);
        }

        Ok(())
    }

    fn read(path: &Path) -> Result<File, Self::Error> {
        match File::open(path) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!(
                    "error",
                    "Failed to open file {}: {}",
                    path.to_str().unwrap_or(""),
                    e
                );
                return Err(e);
            }
        }
    }

    fn delete(path: &Path) -> Result<(), Self::Error> {
        match remove_file(path) {
            Ok(file) => Ok(file),
            Err(e) => {
                log!(
                    "error",
                    "Failed to delete file {}: {}",
                    path.to_str().unwrap_or(""),
                    e
                );
                return Err(e);
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
