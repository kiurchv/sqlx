use crate::net::Socket;

impl Socket for worker::Socket {
    fn try_read(&mut self, buf: &mut dyn crate::io::ReadBuf) -> std::io::Result<usize> {
        todo!()
    }

    fn try_write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn poll_read_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        todo!()
    }

    fn poll_write_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        todo!()
    }

    fn poll_shutdown(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        todo!()
    }
}
