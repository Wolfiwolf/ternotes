mod add;
mod cat;
mod delete;
mod yank;
mod edit;
mod help;
mod list;
mod version;

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    return list::_list();
}

pub fn cat(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    return cat::_cat(note_id);
}

pub fn edit(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    return edit::_edit(note_id);
}

pub fn add(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    return add::_add(name);
}

pub fn delete(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    return delete::_delete(note_id);
}

pub fn yank(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    return yank::_yank(note_id);
}

pub fn help() -> Result<(), Box<dyn std::error::Error>> {
    return help::_help();
}

pub fn version() -> Result<(), Box<dyn std::error::Error>> {
    return version::_version();
}
