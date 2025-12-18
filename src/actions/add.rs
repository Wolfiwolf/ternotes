use crate::{config, notes};

pub fn _add(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let conf = config::get()?;

    let notes = notes::get()?;

    for note in notes {
        if note.name == *name {
            return Err(format!("Note '{}' already exists", name) )?
        }
    }

    let fpath = conf.notes_dir + "/" + name;
    std::fs::File::create(&fpath)?;

    let task = std::process::Command::new(conf.editor_cmd)
        .arg(fpath)
        .spawn();

    if task.is_ok() {
        task.unwrap().wait()?;
        return Ok(());
    }

    let conf = config::get()?;

    return Err(format!(
        "The '{}' note has been created but can't be opened.\n\nDo you have '{}' installed? If not, you\ncan either install it or change the\neditorcmd in {}",
        name,
        conf.editor_cmd,
        config::get_config_filepath()
    ))?;
}
