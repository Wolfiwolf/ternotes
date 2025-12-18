use crate::notes;

pub fn _list() -> Result<(), Box<dyn std::error::Error>> {
    let notes = notes::get()?;

    if notes.iter().count() == 0 {
        println!("\nNo notes exist!\n");

        return Ok(());
    }

    println!("");
    for note in notes {
        println!("{}. {}", note.index, note.name);
    }
    println!("");

    Ok(())
}
