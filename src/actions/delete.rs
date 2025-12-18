use crate::notes;

pub fn _delete(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let notes = notes::get()?;

    if note_id >= notes.iter().count() as i32 {
        return Err("Note id is out of range")?;
    }

    let note = &notes[note_id as usize];

    std::fs::remove_file(&note.path)?;

    return Ok(());
}
