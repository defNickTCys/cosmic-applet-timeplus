// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::env;
use std::path::PathBuf;

/// Obtém o diretório de dados do applet seguindo XDG Base Directory Specification.
///
/// Prioridade:
/// 1. $COSMIC_APPLET_TIMEPLUS_DATA (Dev override)
/// 2. $XDG_DATA_HOME/cosmic-applet-timeplus (Usuário)
/// 3. ~/.local/share/cosmic-applet-timeplus (Fallback usuário)
/// 4. /app/share/cosmic-applet-timeplus (Flatpak)
/// 5. /usr/share/cosmic-applet-timeplus (Sistema)
fn get_data_dir() -> Option<PathBuf> {
    // 1. Environment override (desenvolvimento)
    if let Ok(custom_path) = env::var("COSMIC_APPLET_TIMEPLUS_DATA") {
        return Some(PathBuf::from(custom_path));
    }

    // 2. XDG_DATA_HOME
    if let Some(xdg_data) = dirs::data_dir() {
        let applet_data = xdg_data.join("cosmic-applet-timeplus");
        if applet_data.exists() {
            return Some(applet_data);
        }
    }

    // 3. Fallback ~/.local/share
    if let Some(home) = dirs::home_dir() {
        let local_data = home.join(".local/share/cosmic-applet-timeplus");
        if local_data.exists() {
            return Some(local_data);
        }
    }

    // 4. Flatpak
    let flatpak_path = PathBuf::from("/app/share/cosmic-applet-timeplus");
    if flatpak_path.exists() {
        return Some(flatpak_path);
    }

    // 5. Sistema
    let system_path = PathBuf::from("/usr/share/cosmic-applet-timeplus");
    if system_path.exists() {
        return Some(system_path);
    }

    None
}

/// Obtém o caminho de um asset de áudio.
///
/// # Argumentos
/// * `name` - Nome do arquivo de áudio (ex: "alarm.ogg")
///
/// # Retorno
/// PathBuf com o caminho completo do arquivo. Retorna fallback local se não encontrado.
///
/// # Exemplo
/// ```
/// use cosmic_applet_timeplus::paths::get_audio_path;
/// let alarm_path = get_audio_path("alarm.ogg");
/// println!("Audio at: {:?}", alarm_path);
/// ```
pub fn get_audio_path(name: &str) -> PathBuf {
    // Tenta localizar o diretório de dados
    if let Some(data_dir) = get_data_dir() {
        let audio_path = data_dir.join("sounds").join(name);
        if audio_path.exists() {
            return audio_path;
        }
    }

    // Fallback: diretório local (desenvolvimento)
    PathBuf::from("./assets/sounds").join(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_audio_path_returns_valid_pathbuf() {
        let path = get_audio_path("alarm.ogg");
        assert!(path.to_str().unwrap().contains("alarm.ogg"));
    }

    #[test]
    fn test_respects_env_var_override() {
        // SAFETY: Test function, setting env var before spawning threads
        unsafe {
            env::set_var("COSMIC_APPLET_TIMEPLUS_DATA", "/tmp/test-data");
        }
        let data_dir = get_data_dir();
        assert_eq!(data_dir, Some(PathBuf::from("/tmp/test-data")));
        // SAFETY: Test cleanup
        unsafe {
            env::remove_var("COSMIC_APPLET_TIMEPLUS_DATA");
        }
    }
}
