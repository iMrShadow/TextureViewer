use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileManager {
    pub selected_folder_path: PathBuf,
    pub files: Vec<PathBuf>,
    pub selected_file_index: Option<usize>,
    pub filter_extensions: Vec<String>,
}

impl FileManager {
    /// Creates a new `FileManager` instance initialized to the current directory.
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self {
            selected_folder_path: PathBuf::new(),
            files: Vec::new(),
            selected_file_index: None,
            filter_extensions: Vec::new(),
        })
    }

    /// Initializes the `FileManager` with a specific folder path.
    pub fn from_folder(&mut self, folder_path: PathBuf) -> Result<(), io::Error> {
        if !folder_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Specified path is not a directory",
            ));
        }

        let files = Self::get_files_from_folder(&folder_path, &self.filter_extensions)?;
        let selected_file_index = if files.is_empty() { None } else { Some(0) };

        self.selected_folder_path = folder_path;
        self.files = files;
        self.selected_file_index = selected_file_index;
        Ok(())
    }

    /// Refreshes the list of files in the current folder, applying any set filters.
    pub fn refresh(&mut self) -> Result<(), io::Error> {
        self.files =
            Self::get_files_from_folder(&self.selected_folder_path, &self.filter_extensions)?;
        self.selected_file_index = if self.files.is_empty() { None } else { Some(0) };
        Ok(())
    }

    /// Sets the file extensions to filter by.
    pub fn set_filter_extensions(&mut self, extensions: Vec<String>) {
        self.filter_extensions = extensions.into_iter().map(String::from).collect();
        // Refresh the file list to apply the new filters
        let _ = self.refresh();
    }

    /// Retrieves the currently selected file.
    pub fn get_selected_file(&self) -> Option<&PathBuf> {
        self.selected_file_index
            .and_then(|index| self.files.get(index))
    }

    /// Moves to the next file in the list.
    pub fn next_file(&mut self) {
        if let Some(index) = self.selected_file_index {
            self.selected_file_index = Some((index + 1) % self.files.len());
        }
    }

    /// Moves to the previous file in the list.
    pub fn previous_file(&mut self) {
        if let Some(index) = self.selected_file_index {
            if index == 0 {
                self.selected_file_index = Some(self.files.len() - 1);
            } else {
                self.selected_file_index = Some(index - 1);
            }
        }
    }

    /// Retrieves files from the specified folder, applying optional extension filters.
    fn get_files_from_folder(
        folder_path: &Path,
        filter_extensions: &[String],
    ) -> Result<Vec<PathBuf>, io::Error> {
        if !folder_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The specified path is not a directory",
            ));
        }

        let entries = fs::read_dir(folder_path)?;
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if filter_extensions.is_empty() {
                    files.push(path);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if filter_extensions.contains(&ext.to_string()) {
                        files.push(path);
                    }
                }
            }
        }

        // Sort files alphabetically for consistent navigation
        files.sort();
        Ok(files)
    }

    pub fn set_selected_file(&mut self, path: PathBuf) {
        self.selected_file_index = self.files.iter().position(|p| p == &path);
    }
}
