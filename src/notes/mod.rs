use crate::config;

#[derive(Debug)]
pub struct Note {
    pub name: String,
    pub path: String,
    pub index: i32,
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get()?;
    let note_dir = std::path::Path::new(&config.notes_dir);
    if !note_dir.exists() {
        std::fs::create_dir(note_dir)?;
    }

    Ok(())
}

pub fn get() -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let conf = config::get()?;

    let note_dir = std::path::Path::new(&conf.notes_dir);
    if !note_dir.exists() {
        return Err(format!("Directory {} does not exist!", note_dir.display()))?;
    }

    let files = std::fs::read_dir(note_dir).unwrap();

    let mut notes: Vec<Note> = Vec::new();
    let mut path_i = 0;

    for path in files {
        let raw_path = path.unwrap();

        let note = Note {
            name: String::from(raw_path.file_name().to_str().unwrap()),
            path: String::from(raw_path.path().to_str().unwrap()),
            index: path_i,
        };

        notes.push(note);

        path_i += 1;
    }

    Ok(notes)
}

pub fn get_note_content(note_id: i32) -> Result<String, Box<dyn std::error::Error>> {
    let notes = get()?;

    if note_id >= notes.iter().count() as i32 {
        return Err("Note id is out of range")?;
    }

    let note = &notes[note_id as usize];

    let fcontent = std::fs::read_to_string(&note.path)?;

    Ok(fcontent)
}
