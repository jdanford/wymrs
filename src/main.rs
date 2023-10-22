use wymrs::{App, Result};

pub fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}
