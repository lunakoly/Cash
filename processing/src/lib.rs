use std::process::{Command, Child, Stdio};

use tempfile::NamedTempFile;

pub fn launch_command(arguments: &[&str]) -> std::io::Result<Child> {
    Command::new(arguments[0])
        .args(&arguments[1..])
        .spawn()
}

pub fn launch_pipeline<T: Into<Stdio>>(
    input: Option<T>,
    output: Option<T>,
    arguments: &[&[&str]]
) -> std::io::Result<Child> {
    if arguments.len() == 1 {
        let mut command = Command::new(arguments[0][0]);
        command.args(&arguments[0][1..]);

        if let Some(pipe) = input {
            command.stdin(pipe);
        }

        if let Some(pipe) = output {
            command.stdout(pipe);
        }

        return command.spawn();
    }

    let mut first_command = Command::new(arguments[0][0]);
    first_command.args(&arguments[0][1..]);
    first_command.stdout(Stdio::piped());

    if let Some(pipe) = input {
        first_command.stdin(pipe);
    }

    let mut last_child = first_command.spawn()?;

    for it in 1..arguments.len() - 1 {
        if let Some(last_child_output) = last_child.stdout {
            last_child = Command::new(arguments[it][0])
                .args(&arguments[it][1..])
                .stdin(last_child_output)
                .stdout(Stdio::piped())
                .spawn()?;
        } else {
            return Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe));
        }
    }

    if let Some(last_child_output) = last_child.stdout {
        let mut last_command = Command::new(arguments[arguments.len() - 1][0]);
        last_command.args(&arguments[arguments.len() - 1][1..]);
        last_command.stdin(last_child_output);

        if let Some(pipe) = output {
            last_command.stdout(pipe);
        }

        return last_command.spawn();
    } else {
        return Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe));
    }
}

pub fn launch_input_substitution(arguments: &[&str]) -> std::io::Result<(NamedTempFile, Child)> {
    let file = NamedTempFile::new()?;

    println!("Created a temp file: {:?}", file.path());

    let cloned_handle = file.as_file()
        .try_clone()?;

    let child = Command::new(arguments[0])
        .args(&arguments[1..])
        .stdout(cloned_handle)
        .spawn()?;

    return Ok((file, child));
}

pub fn launch_output_substitution(arguments: &[&str]) -> std::io::Result<(NamedTempFile, Child)> {
    let file = NamedTempFile::new()?;

    println!("Created a temp file: {:?}", file.path());

    let cloned_handle = file.as_file()
        .try_clone()?;

    let child = Command::new(arguments[0])
        .args(&arguments[1..])
        .stdin(cloned_handle)
        .spawn()?;

    return Ok((file, child));
}
