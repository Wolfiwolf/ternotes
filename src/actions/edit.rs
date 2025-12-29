use crate::config;
use crate::notes;

pub fn _edit(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let notes = notes::get()?;

    if note_id >= notes.iter().count() as i32 {
        return Err("Note id is out of range")?;
    }

    let conf = config::get()?;

    let task = std::process::Command::new(&conf.editor_cmd)
        .arg(&notes[note_id as usize].path)
        .spawn();

    if task.is_ok() {
        task.unwrap().wait()?;
        return Ok(());
    }

    return Err(format!(
        "Do you have '{}' installed?\nIf not, you can either install it or change the editor in {}",
        &conf.editor_cmd,
        config::get_config_filepath()
    ))?;
}
