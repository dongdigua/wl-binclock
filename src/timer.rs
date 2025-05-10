use nix::unistd::{pipe, dup2, write};
use chrono::{Local, Timelike};
use std::thread;
use std::time::{Duration, SystemTime};
use std::os::fd::AsRawFd;
use crate::debug;

pub fn initialize_timer() {
    let (read_fd, write_fd) = pipe().unwrap();
    let read_fd_raw = read_fd.as_raw_fd();
    let _ = dup2(read_fd_raw, 0);
    drop(read_fd); // DeepSeek
    thread::spawn(move || loop {
        let fd_ref = &write_fd;
        let diff = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_millis();
        thread::sleep(Duration::from_secs(1) - Duration::from_millis(diff.into()));
        let _ = write(fd_ref, &time_digits());
        debug!("tick");
    });
}

fn time_digits() -> [u8; 7] {
    let mut digits = [0u8; 7];
    let now = Local::now();
    let (hours, minutes, seconds) = (
        now.time().hour() as u8,
        now.time().minute() as u8,
        now.time().second() as u8,
    );
    digits[0] = hours / 10;
    digits[1] = hours % 10;
    digits[2] = minutes / 10;
    digits[3] = minutes % 10;
    digits[4] = seconds / 10;
    digits[5] = seconds % 10;
    digits.iter_mut().for_each(|x| *x += 48);
    digits[6] = b'\n';
    digits
}
