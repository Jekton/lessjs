use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

const PROMPT: &str = ">> ";

pub fn start<R, W>(reader: &mut R, writer: &mut W)
    -> io::Result<()>
    where R: Read + ?Sized, W: Write + ?Sized
{
    let mut buf_reader = BufReader::new(reader);
    let mut buf_writer = BufWriter::new(writer);
    let mut line = String::new();
    loop {
        buf_writer.write(PROMPT.as_bytes())?;
        buf_writer.flush()?;
        line.clear();
        buf_reader.read_line(&mut line)?;
        if line.is_empty() {
            return io::Result::Ok(());
        }
        if line.len() == 1 {
            // only contains a newline
            continue;
        }
        let mut lexer = Lexer::new(&line);
        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::EOF {
                break;
            }
            write!(buf_writer, "{:?}\n", token)?;
        }
        buf_writer.flush()?;
    }
}