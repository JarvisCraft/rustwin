use leptos_i18n_build::{Config, FileFormat, ParseOptions, TranslationsInfos};
use std::{error::Error, path::PathBuf};

type Result = std::result::Result<(), Box<dyn Error>>;

fn main() -> Result {
    setup_i18n()?;
    Ok(())
}

fn setup_i18n() -> Result {
    let mod_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("i18n");
    let cfg = Config::new("ru")?
        .add_locale("en")?
        .parse_options(ParseOptions::default().file_format(FileFormat::Toml));

    let translations_infos = TranslationsInfos::parse(cfg)?;
    translations_infos.emit_diagnostics();
    translations_infos.rerun_if_locales_changed();
    translations_infos.generate_i18n_module(mod_dir)?;

    Ok(())
}
