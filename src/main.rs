extern crate elasticlean;
use elasticlean::indexparser::IndexParser;

fn main() -> Result<(),String> {
    IndexParser::parse("foo-2018.03.02")?;
    Ok(())
}
