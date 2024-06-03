use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn write_greenln(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    writeln!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}

pub fn write_redln(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
    writeln!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}
pub fn write_buleln(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
    writeln!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}

pub fn write_blackln(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)));
    writeln!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}

pub fn write_green(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    write!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}

pub fn write_red(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
    write!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}
pub fn write_bule(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
    write!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}

pub fn write_black(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)));
    write!(&mut stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}
