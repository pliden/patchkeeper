use anyhow::Result;
use anyhow::anyhow;

pub fn format(result: Result<()>) -> Result<()> {
    match result {
        Err(error) => {
            if let Some(error) = error.downcast_ref::<std::io::Error>() {
                let message = error.to_string();
                let trimmed = message
                    .split_once(" (os error")
                    .map(|(first, _)| first)
                    .unwrap_or(&message)
                    .to_lowercase();
                Err(anyhow!(trimmed))
            } else if let Some(error) = error.downcast_ref::<git2::Error>() {
                let trimmed = error.message().trim_end_matches('.').to_string();
                Err(anyhow!(trimmed))
            } else {
                Err(error)
            }
        }
        _ => Ok(()),
    }
}
