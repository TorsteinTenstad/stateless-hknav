use std::process::Command;

pub fn exec_cmd(command: &str) -> Result<(), std::io::Error> {
    Command::new("cmd")
        .args(["/C", command])
        .spawn()
        .map(|_| ())
}
