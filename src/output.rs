use chrono::Local;

use crate::config::OutputFormat;

pub fn generate_filename(raw_url: &str, format: &OutputFormat) -> String {
    let domain = extract_domain(raw_url);
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    format!("{}-{}.{}", domain, timestamp, format.extension())
}

fn extract_domain(raw_url: &str) -> String {
    let parsed = url::Url::parse(raw_url).ok();

    let host = parsed
        .as_ref()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "screenshot".to_string());

    let host = host.strip_prefix("www.").unwrap_or(&host).to_string();

    let mut name = host.replace('.', "-");

    if let Some(path) = parsed.as_ref().map(|u| u.path()) {
        let segment = path
            .trim_matches('/')
            .split('/')
            .next()
            .unwrap_or("")
            .to_string();

        if !segment.is_empty() {
            name = format!("{}-{}", name, sanitize(&segment));
        }
    }

    if name.len() > 80 {
        name.truncate(80);
    }

    name
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}
