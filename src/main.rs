use anyhow::Result;
use fer::run;

fn main() -> Result<()> {
    let res = run();
    println!("res: {:?}", res);
    res.unwrap();
    Ok(())
}
