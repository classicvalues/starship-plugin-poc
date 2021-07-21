use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    io::{BufReader, BufWriter, Read, Write},
    process::{Command, Stdio},
};

use starship_plugin::Message;

fn main() -> Result<(), Box<dyn Error>> {
    let process = Command::new("target/debug/starship-plugin-git")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.expect("Could not capture stdout");
    let mut reader = BufReader::new(stdout);

    let stdin = process.stdin.expect("Could not capture stdin");
    let mut writer = BufWriter::new(stdin);

    let mut size_buffer = [0; 4];
    reader.read_exact(&mut size_buffer)?;
    let size = u32::from_le_bytes(size_buffer);

    let mut msg_buffer = vec![0; size.try_into()?];
    reader.read_exact(&mut msg_buffer)?;

    let message = Message::try_from(msg_buffer)?;
    match message {
        Message::CurrentDir => {
            let current_dir = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let message = bincode::serialize(&current_dir).unwrap();
            let message_size = u32::try_from(message.len())?;

            writer.write_all(&u32::to_le_bytes(message_size))?;
            writer.write_all(&message)?;
            writer.flush()?;
        }
    }

    Ok(())
}