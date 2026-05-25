pub fn get_version_text() -> String {
    // I used a variant of semantic version:
    //   - When launching to production, igone .p an use only M.m
    //   - After launch, increment M.m and start working with M.m.1
    //   - Increment .p as need during M.m development work.
    // https://www.susosise.es/documentos/Una_sugerencia_practica_de_versionado_semantico.pdf

    let parsed_version_result = semver::Version::parse(env!("CARGO_PKG_VERSION"));
    match parsed_version_result {
        Ok(version) => {
            if version.patch == 0 {
                format!("(v{}.{})", version.major, version.minor)
            } else {
                format!(
                    "(v{}.{}.{}-{}-)",
                    version.major,
                    version.minor,
                    t!("previewToTryAndProvideFeedback"),
                    version.patch
                )
            }
        }
        Err(_error) => String::from("(..)"),
    }
}

