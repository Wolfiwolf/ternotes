use crate::notes;

use std::io::Write;

pub fn _delete(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let notes = notes::get()?;

    if note_id >= notes.iter().count() as i32 {
        return Err("Note id is out of range")?;
    }

    let note = &notes[note_id as usize];

    print!(
        "Are you sure you want to delete note '{}' (y/N)? ",
        note.name
    );
    std::io::stdout().flush()?;

    let mut selection = String::new();

    std::io::stdin().read_line(&mut selection)?;

    if !selection.is_empty() {
        if selection.chars().next().unwrap() != 'y' {
            println!("\nDeletion aborted.\n");
            return Ok(());
        }
    }

    std::fs::remove_file(&note.path)?;

    println!("\nThe note '{}' has been deleted.\n", note.name);

    return Ok(());
}
