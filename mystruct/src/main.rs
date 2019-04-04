
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;

use chrono::prelude::DateTime;
use chrono::{Local};
use std::time::{UNIX_EPOCH, Duration};

use std::fmt;

#[derive(Debug)]
struct StatFic {
    filename: String,
    metadata: std::fs::Metadata,
}

impl StatFic {
    fn new(file: &String) -> StatFic {
      StatFic { filename: file.to_string(), metadata: fs::metadata(&file).expect("No file") }
    }

    fn file_type(&self) -> String {
       let ft = &self.metadata.file_type();

       if ft.is_file() {
           "File".to_string()
        } else if ft.is_dir() {
           "Directory".to_string()
        } else if ft.is_symlink() {
           "Symlink".to_string()
        } else if ft.is_block_device() {
           "Block device".to_string()
        } else if ft.is_char_device() {
           "Char device".to_string()
        } else if ft.is_fifo() {
           "FIFO".to_string()
        } else if ft.is_socket() {
           "Socket".to_string()
        } else {
           "desconocido".to_string()
        }
    }

  fn size(&self) -> u64 {
      self.metadata.len()
  }

  fn dev_id(&self) -> u64 {
      self.metadata.dev()
  }

  fn rdev_id(&self) -> u64 {
      self.metadata.rdev()
  }

  fn inode(&self) -> u64 {
      self.metadata.ino()
  }

  fn mode(&self) -> u32 {
      self.metadata.mode()
  }

  fn uid(&self) -> u32 {
      self.metadata.uid()
  }

  fn gid(&self) -> u32 {
      self.metadata.gid()
  }

  fn ctime(&self) -> String {
      let d = UNIX_EPOCH + Duration::from_secs(self.metadata.ctime() as u64);
      DateTime::<Local>::from(d).format("%d/%m/%Y %H:%M:%S").to_string()
  }

  fn mtime(&self) -> String {
      let d = UNIX_EPOCH + Duration::from_secs(self.metadata.mtime() as u64);
      DateTime::<Local>::from(d).format("%d/%m/%Y %H:%M:%S").to_string()
  }

  fn atime(&self) -> String {
      let d = UNIX_EPOCH + Duration::from_secs(self.metadata.atime() as u64);
      DateTime::<Local>::from(d).format("%d/%m/%Y %H:%M:%S").to_string()
  }

  fn hardlink(&self) -> u64 {
      self.metadata.nlink()
  }

  fn blksize(&self) -> u64 {
      self.metadata.blksize()
  }

  fn blocks(&self) -> u64 {
      self.metadata.blocks()
  }

  fn pretty_print(&self) {
    println!("Name........: {}", self.filename);
    println!("Type........: {}", self.file_type());
    println!("Size........: {}", self.size());
    println!("Device......: {}", self.dev_id());
    println!("RDevice.....: {}", self.rdev_id());
    println!("Inode.......: {}", self.inode());
    println!("Mode......xx: {:o}", self.mode());
    println!("UID.........: {}", self.uid());
    println!("GID.........: {}", self.gid());
    println!("Creation..xx: {}", self.ctime());
    println!("Modification: {}", self.mtime());
    println!("Acces.....xx: {}", self.atime());
    println!("Hardlinks...: {}", self.hardlink());
    println!("Blocks/size.: {}/{}", self.blocks(), self.blksize());
  }

}

impl fmt::Display for StatFic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Filename: {}, File_type: {}, Size: {}, Inode: {}, Mode: {:o}, UID: {}, GID: {}, Creation: {}, Modification: {}", self.filename, self.file_type(), self.size(), self.inode(), self.mode(), self.uid(), self.gid(), self.ctime(), self.mtime())
    }
}

fn main() {
    let mific = StatFic::new(&String::from("Cargo.lock"));
    
    mific.pretty_print();
    println!("");
    println!("Fichero: {:?}", mific);
    println!("");
    println!("Fichero: {}", mific);
    println!("");

    let midir = StatFic::new(&String::from("src"));
    
    midir.pretty_print();
    println!("");
    println!("Fichero: {:?}", midir);
    println!("");
    println!("Fichero: {}", midir);
    println!("");

    //Again
    println!("Fichero: {}", mific);
    println!("Fichero: {}", midir);
    println!("");

}
