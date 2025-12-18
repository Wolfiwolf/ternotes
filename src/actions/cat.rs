use crate::notes;

pub fn _cat(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let fcontent = notes::get_note_content(note_id)?;

    println!("\n{}", fcontent);

    Ok(())
}
