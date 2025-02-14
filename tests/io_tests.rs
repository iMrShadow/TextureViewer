use std::fs::File;
use std::io::{self};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use TextureViewer::io::file_manager::FileManager;

    #[test]
    fn test_new_file_manager() -> Result<(), io::Error> {
        let file_manager = FileManager::new()?;
        assert!(!file_manager.selected_folder_path.is_dir());
        assert!(file_manager.files.is_empty());
        assert!(file_manager.filter_extensions.is_empty());
        assert!(file_manager.selected_file_index.is_none());
        Ok(())
    }

    #[test]
    fn test_from_folder() -> Result<(), io::Error> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");
        File::create(&file_path)?;

        let mut file_manager = FileManager::new().unwrap();
        file_manager.from_folder(dir.path().to_path_buf())?;

        assert_eq!(file_manager.selected_folder_path, dir.path());
        assert_eq!(file_manager.files.len(), 1);
        assert_eq!(file_manager.files[0], file_path);
        assert_eq!(file_manager.selected_file_index, Some(0));
        Ok(())
    }

    #[test]
    fn test_set_filter_extensions() -> Result<(), io::Error> {
        let dir = tempdir()?;
        File::create(dir.path().join("image1.jpg"))?;
        File::create(dir.path().join("image2.png"))?;
        File::create(dir.path().join("test.txt"))?;

        let mut file_manager = FileManager::new()?;
        file_manager.from_folder(dir.path().to_path_buf())?;
        file_manager.set_filter_extensions(vec!["jpg".to_string(), "png".to_string()]);

        assert_eq!(file_manager.files.len(), 2);
        assert!(file_manager.files.iter().all(|file| {
            let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");
            ext == "jpg" || ext == "png"
        }));
        Ok(())
    }

    #[test]
    fn test_navigation() -> Result<(), io::Error> {
        let dir = tempdir()?;
        File::create(dir.path().join("test1.txt"))?;
        File::create(dir.path().join("test2.txt"))?;
        File::create(dir.path().join("test3.txt"))?;

        let mut file_manager = FileManager::new().unwrap();

        file_manager.from_folder(dir.path().to_path_buf())?;
        // Initial selection
        assert_eq!(
            file_manager.get_selected_file(),
            Some(&dir.path().join("test1.txt"))
        );

        // Next
        file_manager.next_file();
        assert_eq!(
            file_manager.get_selected_file(),
            Some(&dir.path().join("test2.txt"))
        );

        // Next
        file_manager.next_file();
        assert_eq!(
            file_manager.get_selected_file(),
            Some(&dir.path().join("test3.txt"))
        );

        // Clamp to first file
        file_manager.next_file();
        assert_eq!(
            file_manager.get_selected_file(),
            Some(&dir.path().join("test1.txt"))
        );

        // Clamp to last file
        file_manager.previous_file();
        assert_eq!(
            file_manager.get_selected_file(),
            Some(&dir.path().join("test3.txt"))
        );

        Ok(())
    }
}
