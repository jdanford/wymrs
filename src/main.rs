use anyhow::Result;
use wymrs::App;

pub fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}
