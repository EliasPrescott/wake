use std::io::{Read, Write};

pub struct WrapperReader<R> {
    start_wrapper: Vec<u8>,
    child_reader: R,
    end_wrapper: Vec<u8>,
}

impl<R> WrapperReader<R>
where
    R: Read,
{
    pub fn new(child_reader: R, start_wrapper: &str, end_wrapper: &str) -> Self {
        WrapperReader {
            start_wrapper: start_wrapper.into(),
            child_reader,
            end_wrapper: end_wrapper.into(),
        }
    }
}

impl<R> Read for WrapperReader<R>
where
    R: Read,
{
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let temp_buf = &mut vec![0; 8_000];
        let read_result = self.child_reader.read(temp_buf)?;

        if read_result > 0 {
            let mut out = self.start_wrapper.clone();
            out.append(temp_buf);
            out.append(&mut self.end_wrapper.clone());
            buf.write(&out)
        } else {
            Ok(0)
        }
    }
}
