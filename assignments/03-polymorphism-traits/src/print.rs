use anyhow::Result;
use colored::{Color, Colorize};
use regex::Regex;
use std::io::Write;

pub trait Printer<W: Write> {
    fn print(&mut self, line: &str, needle: &str) -> Result<()>;
}

pub struct PlainPrinter<W: Write> {
    writer: W,
}

impl<W: Write> PlainPrinter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> Printer<W> for PlainPrinter<W> {
    fn print(&mut self, line: &str, _needle: &str) -> Result<()> {
        writeln!(self.writer, "{}", line)?;
        Ok(())
    }
}

pub struct ColoredPrinter<W: Write> {
    writer: W,
    color: Color,
}

impl<W: Write> ColoredPrinter<W> {
    pub fn new(writer: W, color: Color) -> Self {
        Self { writer, color }
    }
}

impl<W: Write> Printer<W> for ColoredPrinter<W> {
    fn print(&mut self, line: &str, needle: &str) -> Result<()> {
        // Cceck if valid regex
        if let Ok(regex) = Regex::new(needle) {
            // use regex to find matches & apply color
            let colored_line = regex.replace_all(line, |caps: &regex::Captures| {
                // colorize
                caps[0].color(self.color).to_string()
            });
            writeln!(self.writer, "{}", colored_line)?;
        } else {
            let colored_line = line.replace(needle, &needle.color(self.color).to_string());
            writeln!(self.writer, "{}", colored_line)?;
        }
        Ok(())
    }
}
