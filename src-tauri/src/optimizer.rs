use std::path::PathBuf;

#[cfg(windows)]
use std::fs;

#[cfg(windows)]
pub fn config_dir() -> PathBuf {
    let dirs = directories::ProjectDirs::from("com", "optiq", "Optiq")
        .expect("Could not determine config directory");
    dirs.config_dir().to_path_buf()
}

#[cfg(windows)]
fn key_path() -> PathBuf {
    config_dir().join("api_key.enc")
}

#[cfg(windows)]
mod dpapi {
    use base64::{engine::general_purpose::STANDARD as B64, Engine};
    use std::fs;
    use std::path::Path;
    use windows::Win32::Security::Cryptography::{
        CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB,
    };

    fn dpapi_encrypt(plain: &str) -> Result<Vec<u8>, String> {
        let bytes = plain.as_bytes();
        let input = CRYPT_INTEGER_BLOB {
            cbData: bytes.len() as u32,
            pbData: bytes.as_ptr() as *mut u8,
        };
        let mut output = CRYPT_INTEGER_BLOB {
            cbData: 0,
            pbData: std::ptr::null_mut(),
        };

        let result = unsafe { CryptProtectData(&input, None, None, None, None, 0, &mut output) };

        result.map_err(|e| format!("CryptProtectData failed: {}", e))?;

        let encrypted =
            unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
        unsafe {
            let _ = windows::Win32::System::Memory::LocalAlloc(
                windows::Win32::System::Memory::LMEM_FIXED,
                0,
            );
        }
        Ok(encrypted)
    }

    fn dpapi_decrypt(encrypted: &[u8]) -> Result<String, String> {
        let input = CRYPT_INTEGER_BLOB {
            cbData: encrypted.len() as u32,
            pbData: encrypted.as_ptr() as *mut u8,
        };
        let mut output = CRYPT_INTEGER_BLOB {
            cbData: 0,
            pbData: std::ptr::null_mut(),
        };

        let result = unsafe { CryptUnprotectData(&input, None, None, None, None, 0, &mut output) };

        result.map_err(|e| format!("CryptUnprotectData failed: {}", e))?;

        let plain = unsafe {
            let slice = std::slice::from_raw_parts(output.pbData, output.cbData as usize);
            String::from_utf8_lossy(slice).to_string()
        };
        Ok(plain)
    }

    pub fn save(key: &str, path: &Path) -> Result<(), String> {
        let encrypted = dpapi_encrypt(key.trim())?;
        let encoded = B64.encode(&encrypted);
        fs::write(path, encoded).map_err(|e| e.to_string())
    }

    pub fn load(path: &Path) -> Option<String> {
        let encoded = fs::read_to_string(path).ok()?;
        let encrypted = B64.decode(encoded.trim()).ok()?;
        let plain = dpapi_decrypt(&encrypted).ok()?;
        if plain.trim().is_empty() {
            None
        } else {
            Some(plain)
        }
    }

    pub fn delete(path: &Path) -> Result<(), String> {
        if path.exists() {
            fs::remove_file(path).map_err(|e| e.to_string())
        } else {
            Ok(())
        }
    }
}

#[cfg(not(windows))]
mod dpapi {
    const SERVICE: &str = "com.optiq";
    const ACCOUNT: &str = "api-key";

    pub fn save(key: &str) -> Result<(), String> {
        let entry = keyring::Entry::new(SERVICE, ACCOUNT).map_err(|e| e.to_string())?;
        entry.set_password(key.trim()).map_err(|e| e.to_string())
    }

    pub fn load() -> Option<String> {
        let entry = keyring::Entry::new(SERVICE, ACCOUNT).ok()?;
        entry.get_password().ok()
    }

    pub fn delete() -> Result<(), String> {
        let entry = keyring::Entry::new(SERVICE, ACCOUNT).map_err(|e| e.to_string())?;
        entry.delete_credential().map_err(|e| e.to_string())
    }
}

pub fn load_api_key() -> Option<String> {
    #[cfg(windows)]
    {
        dpapi::load(&key_path())
    }
    #[cfg(not(windows))]
    {
        dpapi::load()
    }
}

pub fn save_api_key(key: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        let dir = config_dir();
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        dpapi::save(key, &key_path())
    }
    #[cfg(not(windows))]
    {
        dpapi::save(key)
    }
}

pub fn delete_api_key() -> Result<(), String> {
    #[cfg(windows)]
    {
        dpapi::delete(&key_path())
    }
    #[cfg(not(windows))]
    {
        dpapi::delete()
    }
}
