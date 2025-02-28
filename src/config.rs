use anyhow::{anyhow, Error};
use dirs::config_dir;
use ini::Ini;
use rat_widget::text::Locale;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::str::FromStr;
use sys_locale::get_locale;

#[derive(Debug)]
pub struct MDConfig {
    // system
    pub loc: Locale,

    // ui config
    pub theme: String,
    pub text_width: u16,

    // startup
    pub load_file: Vec<PathBuf>,
    pub globs: Vec<String>,

    // auto/tmp
    pub file_split_at: u16,
    pub show_ctrl: bool,
    pub log_level: String,
}

const DEFAULT_FILE_SPLIT_AT: u16 = 15;
const DEFAULT_TEXT_WIDTH: u16 = 65;

impl Default for MDConfig {
    fn default() -> Self {
        let loc = get_locale().unwrap_or("en-US".into()).replace('-', "_");
        let locale = Locale::from_str(&loc).unwrap_or(Locale::POSIX);

        MDConfig {
            loc: locale,
            theme: "Imperial".to_string(),
            show_ctrl: false,
            file_split_at: DEFAULT_FILE_SPLIT_AT,
            text_width: DEFAULT_TEXT_WIDTH,
            load_file: Default::default(),
            globs: vec!["*.md".to_string()],
            log_level: "debug".to_string(),
        }
    }
}

impl MDConfig {
    pub fn load() -> Result<MDConfig, Error> {
        let cfg = if let Some(config) = config_dir() {
            let config = config.join("mdedit").join("mdedit.ini");
            if config.exists() {
                let ini = Ini::load_from_file(config)?;

                let section: Option<String> = None;

                let theme = ini.get_from_or(section.clone(), "theme", "Imperial");

                let file_split_at = ini
                    .get_from_or(
                        section.clone(),
                        "file_split_at",
                        DEFAULT_FILE_SPLIT_AT.to_string().as_str(),
                    )
                    .parse()
                    .unwrap_or(DEFAULT_FILE_SPLIT_AT);

                let text_width = ini
                    .get_from_or(
                        section.clone(),
                        "text_width",
                        DEFAULT_TEXT_WIDTH.to_string().as_str(),
                    )
                    .parse()
                    .unwrap_or(DEFAULT_TEXT_WIDTH);

                let mut globs = ini
                    .get_from_or(section.clone(), "file_pattern", "*.md")
                    .split([' ', ','])
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>();
                globs.sort();
                globs.dedup();

                let log = ini
                    .get_from_or(section.clone(), "log", "warn")
                    .trim()
                    .to_string();

                Some(MDConfig {
                    theme: theme.into(),
                    file_split_at,
                    text_width,
                    globs,
                    log_level: log,
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            None
        };

        Ok(cfg.unwrap_or(MDConfig::default()))
    }

    pub fn store(&self) -> Result<(), Error> {
        if let Some(config_root) = config_dir() {
            let config_dir = config_root.join("mdedit");
            if !config_dir.exists() {
                create_dir_all(&config_dir)?;
            }

            let config = config_dir.join("mdedit.ini");
            let mut ini = Ini::new();
            let section: Option<String> = None;
            ini.set_to(section.clone(), "theme".into(), self.theme.clone());
            ini.set_to(
                section.clone(),
                "file_split_at".into(),
                self.file_split_at.to_string(),
            );
            ini.set_to(
                section.clone(),
                "text_width".into(),
                self.text_width.to_string(),
            );
            ini.set_to(
                section.clone(),
                "file_pattern".into(),
                self.globs
                    .iter()
                    .cloned()
                    .reduce(|mut v, w| {
                        v.push(',');
                        v.push(' ');
                        v.push_str(&w);
                        v
                    })
                    .unwrap_or("*.md".to_string()),
            );
            ini.set_to(section.clone(), "log".into(), self.log_level.clone());

            ini.write_to_file(config)?;

            Ok(())
        } else {
            Err(anyhow!("Can't save config."))
        }
    }
}
