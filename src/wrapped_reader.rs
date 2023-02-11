use std::{pin::Pin, task::Poll};

use tokio::io::AsyncRead;

pub struct WrapperReader<R> {
    start_wrapper: Vec<u8>,
    child_reader: Pin<Box<R>>,
    end_wrapper: Vec<u8>,
}

impl<R> WrapperReader<R>
where
    R: AsyncRead,
{
    pub fn new(child_reader: R, start_wrapper: &str, end_wrapper: &str) -> Self {
        WrapperReader {
            start_wrapper: start_wrapper.into(),
            child_reader: Box::pin(child_reader),
            end_wrapper: end_wrapper.into(),
        }
    }
}

impl<R> AsyncRead for WrapperReader<R>
where
    R: AsyncRead,
{
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let temp_buf = &mut vec![0; 8_000];
        let async_read_result = self
            .child_reader
            .as_mut()
            .poll_read(cx, &mut tokio::io::ReadBuf::new(temp_buf));
        match async_read_result {
            Poll::Ready(Ok(())) => {
                let mut out = self.start_wrapper.clone();
                out.append(temp_buf);
                out.append(&mut self.end_wrapper.clone());
                let mut out_reader = tokio::io::BufReader::new(out.as_ref());
                let p = Pin::new(&mut out_reader);
                p.poll_read(cx, buf)
            }
            other => other,
        }
    }
}
