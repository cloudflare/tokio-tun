use crate::linux::interface::Interface;
use crate::linux::params::Params;
use crate::result::Result;
use std::ffi::CString;
use std::io;
use std::net::Ipv4Addr;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{self, Context};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// Represents a Tun/Tap device. Use [`TunBuilder`](struct.TunBuilder.html) to create a new instance of [`Tun`](struct.Tun.html).
pub struct Tun {
    iface: Arc<Interface>,
    io: File,
}

impl AsRawFd for Tun {
    fn as_raw_fd(&self) -> RawFd {
        self.io.as_raw_fd()
    }
}

impl AsyncRead for Tun {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().io).poll_read(cx, buf)
    }
}

impl AsyncWrite for Tun {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> task::Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().io).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> task::Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().io).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> task::Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().io).poll_shutdown(cx)
    }
}

impl Tun {
    /// Creates a new instance of Tun/Tap device.
    pub(crate) fn new(params: Params) -> Result<Self> {
        let iface = Self::allocate(params, 1)?;
        let fd = iface.files()[0];
        Ok(Self {
            iface: Arc::new(iface),
            io: unsafe { File::from_raw_fd(fd) },
        })
    }

    /// Creates a new instance of Tun/Tap device.
    pub(crate) fn new_mq(params: Params, queues: usize) -> Result<Vec<Self>> {
        let iface = Self::allocate(params, queues)?;
        let mut tuns = Vec::with_capacity(queues);
        let files = iface.files().to_vec();
        let iface = Arc::new(iface);
        for fd in files.into_iter() {
            tuns.push(Self {
                iface: iface.clone(),
                io: unsafe { File::from_raw_fd(fd) },
            })
        }
        Ok(tuns)
    }

    fn allocate(params: Params, queues: usize) -> Result<Interface> {
        let mut fds = Vec::with_capacity(queues);
        let path = CString::new("/dev/net/tun")?;
        for _ in 0..queues {
            fds.push(unsafe { libc::open(path.as_ptr(), libc::O_RDWR) });
        }
        let iface = Interface::new(
            fds,
            params.name.as_deref().unwrap_or_default(),
            params.flags,
        )?;
        iface.init(params)?;
        Ok(iface)
    }

    /// Returns the name of Tun/Tap device.
    pub fn name(&self) -> &str {
        self.iface.name()
    }

    /// Returns the value of MTU.
    pub fn mtu(&self) -> Result<i32> {
        self.iface.mtu(None)
    }

    /// Returns the IPv4 address of MTU.
    pub fn address(&self) -> Result<Ipv4Addr> {
        self.iface.address(None)
    }

    /// Returns the IPv4 destination address of MTU.
    pub fn destination(&self) -> Result<Ipv4Addr> {
        self.iface.destination(None)
    }

    /// Returns the IPv4 broadcast address of MTU.
    pub fn broadcast(&self) -> Result<Ipv4Addr> {
        self.iface.broadcast(None)
    }

    /// Returns the IPv4 netmask address of MTU.
    pub fn netmask(&self) -> Result<Ipv4Addr> {
        self.iface.netmask(None)
    }

    /// Returns the flags of MTU.
    pub fn flags(&self) -> Result<i16> {
        self.iface.flags(None)
    }
}
