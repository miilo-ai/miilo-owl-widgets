use crate::Result;
use sha2::{Digest, Sha256};
use std::{
    ffi::CString,
    fs::File,
    io::{Read, Write},
    os::fd::{AsRawFd, FromRawFd},
    path::{Component, Path},
};

pub struct Directory(File);
fn c(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| "Invalid removal path".into())
}
impl Directory {
    pub fn open(path: &Path) -> Result<Self> {
        if !path.is_absolute() {
            return Err("Removal paths must be absolute".into());
        }
        let mut dir = File::open("/").map_err(|_| "Cannot open filesystem root")?;
        for part in path.components() {
            let Component::Normal(name) = part else {
                if matches!(part, Component::RootDir) {
                    continue;
                }
                return Err("Unnormalized removal path".into());
            };
            let name = c(name.to_str().ok_or("Invalid removal path")?)?;
            let fd = unsafe {
                libc::openat(
                    dir.as_raw_fd(),
                    name.as_ptr(),
                    directory_access() | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                )
            };
            if fd < 0 {
                return Err("Removal directory is missing, inaccessible or symlinked".into());
            }
            dir = unsafe { File::from_raw_fd(fd) };
        }
        Ok(Self(dir))
    }
    pub fn read(&self, name: &str, limit: u64) -> Result<Option<Vec<u8>>> {
        let name = c(name)?;
        let fd = unsafe {
            libc::openat(
                self.0.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            if std::io::Error::last_os_error().kind() == std::io::ErrorKind::NotFound {
                return Ok(None);
            }
            return Err("Cannot inspect owned installation file".into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let meta = file.metadata().map_err(|_| "Cannot inspect owned file")?;
        if !meta.is_file() || meta.len() > limit {
            return Err("Invalid owned installation file".into());
        }
        let mut bytes = vec![];
        file.take(limit + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "Cannot read owned file")?;
        if bytes.len() as u64 > limit {
            return Err("Owned file is too large".into());
        }
        Ok(Some(bytes))
    }
    pub fn write(&self, name: &str, bytes: &[u8]) -> Result<()> {
        let temporary = format!(".miilo-receipt-{}", std::process::id());
        let temp = c(&temporary)?;
        let fd = unsafe {
            libc::openat(
                self.0.as_raw_fd(),
                temp.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err("Cannot stage installation receipt".into());
        }
        let mut file = unsafe { File::from_raw_fd(fd) };
        let result = (|| {
            file.write_all(bytes)
                .and_then(|_| file.sync_all())
                .map_err(|_| "Cannot save installation receipt")?;
            let name = c(name)?;
            if unsafe {
                libc::renameat(
                    self.0.as_raw_fd(),
                    temp.as_ptr(),
                    self.0.as_raw_fd(),
                    name.as_ptr(),
                )
            } != 0
            {
                return Err("Cannot publish installation receipt".into());
            }
            Ok(())
        })();
        unsafe {
            libc::unlinkat(self.0.as_raw_fd(), temp.as_ptr(), 0);
        }
        result
    }
    pub fn remove_widget(&self, expected: &str) -> Result<()> {
        let source = c("70-miilo-fleet.owl")?;
        let pending = format!(".miilo-removal-{}", std::process::id());
        let target = c(&pending)?;
        // Reserve the destination; never overwrite an existing recovery file.
        let fd = unsafe {
            libc::openat(
                self.0.as_raw_fd(),
                target.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err("Removal recovery file already exists; inspect it first".into());
        }
        drop(unsafe { File::from_raw_fd(fd) });
        if unsafe {
            libc::renameat(
                self.0.as_raw_fd(),
                source.as_ptr(),
                self.0.as_raw_fd(),
                target.as_ptr(),
            )
        } != 0
        {
            unsafe {
                libc::unlinkat(self.0.as_raw_fd(), target.as_ptr(), 0);
            }
            return Err("Widget changed before removal; inspect a fresh plan".into());
        }
        let valid = self
            .read(&pending, 2_000_000)
            .map_err(|_| "Cannot verify staged widget; .miilo-removal-PID recovery file retained")?
            .is_some_and(|bytes| hash(&bytes) == expected);
        if !valid {
            return Err("Widget changed during removal; preserved in .miilo-removal-PID for manual recovery".into());
        }
        if unsafe { libc::unlinkat(self.0.as_raw_fd(), target.as_ptr(), 0) } != 0 {
            return Err("Cannot remove staged widget; recovery file retained".into());
        }
        Ok(())
    }
}
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(target_os = "linux")]
fn directory_access() -> i32 {
    libc::O_PATH
}
#[cfg(not(target_os = "linux"))]
fn directory_access() -> i32 {
    libc::O_RDONLY
}
