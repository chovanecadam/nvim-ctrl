use anyhow::Result;
use neovim_lib::{Neovim, NeovimApi, Session};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Control nvim from the CLI!")]
struct Control {
    /// run an arbitrary command
    cmd: String,
}

fn main() -> Result<()> {
    let args = Control::from_args();

    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or("/tmp".to_string())
        .to_owned();

    match std::fs::read_dir(runtime_dir) {
        Ok(dir) => dir
            .filter_map(|f| f.ok())
            .filter(|f| f.file_name().to_string_lossy().starts_with("nvim"))
            .filter_map(|f| Session::new_unix_socket(f.path()).ok())
            .map(|mut session| {
                session.start_event_loop();
                Neovim::new(session)
            })
            .for_each(|mut nvim| {
                let _ = nvim
                    .command(&args.cmd)
                    .map_err(|e| eprintln!("Error: {}", e));
            }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => Err(e)?,
    }

    Ok(())
}
