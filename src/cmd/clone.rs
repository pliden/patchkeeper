use crate::stdout;
use anyhow::bail;
use anyhow::Result;
use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use immargs::ImmArgs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[derive(ImmArgs)]
pub struct Args {
    #[cmdline(positional)]
    url: String,

    #[cmdline(positional)]
    path: Option<PathBuf>,
}

fn clone(url: &str, path: Option<PathBuf>) -> Result<()> {
    let path = match path {
        Some(path) => path,
        _ => match PathBuf::from(url).file_stem() {
            Some(stem) => PathBuf::from(stem),
            _ => bail!("cannot resolve directory from URL"),
        },
    };

    stdout!("clone: {}\n", url);

    let mut remote_messages = vec![];
    let mut downloading = true;
    let mut remote_cb = RemoteCallbacks::new();

    remote_cb.sideband_progress(|data| {
        let message = str::from_utf8(data).unwrap_or("???\n").to_string();
        remote_messages.push(message);
        true
    });

    remote_cb.transfer_progress(|stats| {
        if downloading {
            let received = stats.received_objects();
            let total = stats.total_objects();
            let percent = (100 * received) / total;
            stdout!("downloading..... {}%\r", percent);
            if received == total {
                downloading = false;
                stdout!("\n");
            }
        } else {
            let indexed = stats.indexed_objects();
            let total = stats.total_objects();
            let percent = (100 * indexed) / total;
            stdout!("indexing..... {}%\r", percent);
            if indexed == total {
                stdout!("\n");
            }
        }

        let _ = io::stdout().flush();
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_cb);

    let mut checkout_builder = CheckoutBuilder::new();
    checkout_builder.progress(|_, checkedout, total| {
        let percent = (100 * checkedout) / total;
        stdout!("checking out..... {}%\r", percent);
        if checkedout == total {
            stdout!("\n");
        }

        let _ = io::stdout().flush();
    });

    let result = RepoBuilder::new()
        .fetch_options(fetch_options)
        .with_checkout(checkout_builder)
        .clone(url, &path);

    if let Err(error) = result {
        for message in remote_messages {
            stdout!("remote: {}", message);
        }

        bail!(error);
    }

    Ok(())
}

pub fn main(_path: &Path, args: Args) -> Result<()> {
    clone(&args.url, args.path)
}
