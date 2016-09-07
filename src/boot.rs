use std::sync::mpsc::{channel, Sender, Receiver};
use gfx::{Gfx, Pixel};

pub fn gfx() -> (Sender<Vec<Pixel>>, Gfx) {
    let (transmitter, port): (Sender<Vec<Pixel>>, Receiver<Vec<Pixel>>) = channel();

    let gfx = Gfx { port: port };

    (transmitter, gfx)
}
