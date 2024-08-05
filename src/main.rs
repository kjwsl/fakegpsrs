use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    const SAMPLE_PATH: &str = "src/gpslog.log";
    const PIPE_PATH: &str = "/tmp/nmea_pipe";

    if !Path::new(PIPE_PATH).exists() {
        mkfifo(PIPE_PATH, Mode::S_IRUSR | Mode::S_IWUSR).expect("Faild to create named pipe");
    }

    // Read the NMEA sentences from the sample file
    let file = File::open(SAMPLE_PATH)?;
    let reader = BufReader::new(file);
    let nmea_lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut i = 0;
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(PIPE_PATH)?;
    loop {
        // Open the output file in write mode to overwrite it

        // Write the next 5 lines to the output file
        let one_phase = &nmea_lines[i..i + 6];
        for line in one_phase {
            output.write(line.as_bytes());
            output.write("\n\n".as_bytes());
        }
        output.flush()?;

        i += 6;
        // If we reach the end of the file, start over
        if i >= nmea_lines.len() {
            i = 0;
        }

        // Sleep for 1 second before writing the next batch
        thread::sleep(Duration::from_secs(1));
    }
}
