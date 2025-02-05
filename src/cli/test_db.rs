use crate::db::sqlite;

pub fn handle() {
    let _result = sqlite::main();
    println!("Testing DB...")
}
