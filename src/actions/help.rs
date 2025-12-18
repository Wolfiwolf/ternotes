pub fn _help() -> Result<(), Box<dyn std::error::Error>> {
    println!("");
    println!("### Ternotes v{} ###", env!("CARGO_PKG_VERSION"));
    println!("");
    println!("Actions:");
    println!("\tList:    nts               # List the existing notes");
    println!("\tVersion: nts v             # Print out version");
    println!("\tHelp:    nts h             # Print out help");
    println!("\tCat:     nts <note_id>     # Print out the note");
    println!("\tEdit:    nts <note_id> e   # Edit the note");
    println!("\tDelete:  nts <note_id> d   # Delete a note");
    println!("\tAdd:     nts a <note_name> # Add a note");
    println!("");

    Ok(())
}
