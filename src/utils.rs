use std::process::Command;

pub fn exec(command: &mut Command) -> Option<String> {
    let result = command.output().ok()?;
    
    if !result.status.success() {
        return None;
    }
    
    let result_str = String::from_utf8(result.stdout).ok()?;
    Some(result_str)
}
