mod domain;
mod infrastructure;
mod repositories;

pub fn do_stuff() -> Result<(), String> {
    print!("Hello world");

    Ok(())
}
