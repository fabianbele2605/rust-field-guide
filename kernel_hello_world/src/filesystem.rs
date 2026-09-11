const MAX_FILES: usize = 16;
const MAX_FILE_SIZE: usize = 256;
const MAX_NAME_LEN: usize = 32;

#[derive(Clone, Copy)]
pub struct File {
    pub name: [u8; MAX_NAME_LEN],
    pub name_len: usize,
    pub data: [u8; MAX_FILE_SIZE],
    pub data_len: usize,
}

impl File {
    pub fn empty() -> Self {
        File {
            name: [0; MAX_NAME_LEN],
            name_len: 0,
            data: [0; MAX_FILE_SIZE],
            data_len: 0,
        }
    }
}

pub struct FileSystem {
    files: [Option<File>; MAX_FILES],
}

impl FileSystem {
    pub const fn new() -> Self {
        FileSystem {
            files: [None; MAX_FILES],
        }
    }

    pub fn create_file(&mut self, name: &[u8], data: &[u8]) -> bool {
        for slot in self.files.iter_mut() {
            if slot.is_none() {
                let mut file = File::empty();

                let n = name.len().min(MAX_NAME_LEN);
                file.name[..n].copy_from_slice(&name[..n]);
                file.name_len = n;

                let d = data.len().min(MAX_FILE_SIZE);
                file.data[..d].copy_from_slice(&data[..d]);
                file.data_len = d;

                *slot = Some(file);
                return true;
            }
        }
        false // no hay espacio libre
    }

    pub fn read_file(&self, name: &[u8]) -> Option<&[u8]> {
        for slot in self.files.iter() {
            if let Some(file) = slot {
                if &file.name[..file.name_len] == name {
                    return Some(&file.data[..file.data_len]);
                }
            }
        }
        None
    }

    pub fn list_files(&self) -> impl Iterator<Item = &[u8]> {
        self.files.iter().filter_map(|slot| {
            slot.as_ref().map(|file| &file.name[..file.name_len])
        })
    }
}