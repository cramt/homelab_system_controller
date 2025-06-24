use std::{ops::Deref, process::Stdio, sync::Arc};

use bytes::Bytes;
use clone_stream::{CloneStream, ForkStream};
use futures::{Stream, StreamExt};
use tokio::{io::BufReader, process::Command};
use tokio_util::io::ReaderStream;

pub fn new_stream(s: &str) -> ReaderStream<BufReader<tokio::process::ChildStdout>> {
    let mut ffmpeg = Command::new("ffmpeg")
        .arg("-re")
        .arg("-i")
        .arg(s)
        .arg("-g")
        .arg("52")
        .arg("-f")
        .arg("mpegts")
        .arg("-codec:v")
        .arg("libx264")
        .arg("-movflags")
        .arg("frag_keyframe+empty_moov")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    ReaderStream::new(BufReader::new(ffmpeg.stdout.take().unwrap()))
}
