#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
    pub text: String,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("Invalid SPDX expression: {reason}")]
    SpdxParse {
        #[source_code]
        src: miette::NamedSource<String>,

        #[label]
        span: miette::SourceSpan,

        reason: String,
    },

    #[error("Unknown license '{id}' is not permitted")]
    UnknownLicense { id: String },

    #[error("Empty")]
    Empty,
}

impl License {
    pub fn parse_and_resolve(
        spdx_expr_input: &str,
        permit_unknown: bool,
    ) -> Result<Vec<Self>, Error> {
        let expr = spdx::Expression::parse(spdx_expr_input).map_err(|e| Error::SpdxParse {
            src: miette::NamedSource::new("license", spdx_expr_input.to_string()),
            span: miette::SourceSpan::from(e.span),
            reason: e.reason.to_string(),
        })?;

        let mut out = Vec::new();

        for req in expr.requirements() {
            let id = match &req.req.license {
                spdx::LicenseItem::Spdx { id, .. } => id.name, // e.g. 'MIT'
                spdx::LicenseItem::Other(license_ref) => {
                    let id = license_ref.lic_ref.as_str();

                    if !permit_unknown {
                        return Err(Error::UnknownLicense { id: id.to_string() });
                    }

                    out.push(License {
                        id: id.to_string(),
                        name: id.to_string(),
                        url: String::new(),
                        text: String::new(),
                    });

                    continue;
                }
            };

            let entry = match id.parse::<&dyn license::License>() {
                Ok(lic) => License {
                    id: id.to_string(),
                    name: lic.name().to_string(),
                    url: format!("https://spdx.org/licenses/{id}.html"),
                    text: lic.text().to_string(),
                },
                Err(_) if permit_unknown => License {
                    id: id.to_string(),
                    name: id.to_string(),
                    url: format!("https://spdx.org/licenses/{id}.html"),
                    text: String::new(),
                },
                Err(_) => return Err(Error::UnknownLicense { id: id.to_string() }),
            };

            out.push(entry);
        }

        Ok(out)
    }
}
