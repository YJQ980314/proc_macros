use builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("find")
        .args(vec!["-c".into(), "-vvv".into()])
        .env(vec![])
        .current_dir("/home/yanlien/Program/Rust/proc_macros")
        .finish()
        .unwrap();

    println!("{:?}", command);
}
