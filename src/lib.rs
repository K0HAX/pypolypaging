#![allow(non_local_definitions)]

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use polypaging;
use polypaging::packet::Packet;

/// Session object
#[pyclass]
#[derive(Clone, Debug)]
struct SessionInfo {
    #[pyo3(get, set)]
    channelnum: u8,
    #[pyo3(get, set)]
    hostserial: u32,
    callerid_len: u8,
    callerid: String,
}

#[pymethods]
impl SessionInfo {
    #[new]
    fn new(channelnum: u8, hostserial: u32, callerid: String) -> PyResult<Self> {
        if callerid.len() > 13 {
            Err(PyValueError::new_err("The Caller ID must be 13 characters or fewer"))
        } else {
            Ok(Self{
                channelnum: channelnum,
                hostserial: hostserial,
                callerid_len: u8::try_from(callerid.len())?,
                callerid: callerid})
        }
    }

    #[getter]
    fn get_callerid(&self) -> PyResult<String> {
        Ok(self.callerid.clone())
    }

    #[setter]
    fn set_callerid(&mut self, value: String) -> PyResult<()> {
        if value.len() > 13 {
            Err(PyValueError::new_err("The Caller ID must be 13 characters or fewer"))
        } else {
            self.callerid_len = u8::try_from(value.clone().len())?;
            self.callerid = value;
            Ok(())
        }
    }
}

impl SessionInfo {
    fn get_real_session(&self) -> Result<polypaging::session::SessionInfo<'_>, Box<dyn std::error::Error>> {
        Ok(polypaging::session::SessionInfo {
            channelnum: self.channelnum,
            hostserial: self.hostserial,
            callerid_len: self.callerid_len,
            callerid: &self.callerid
        })
    }
}

/// Codec enum
#[pyclass]
#[derive(Clone, Debug)]
enum CodecFlag {
    G711u,
    G722,
}

impl CodecFlag {
    fn to_codec_flag(&self) -> polypaging::rtpcodec::CodecFlag {
        match self {
            CodecFlag::G711u => polypaging::rtpcodec::CodecFlag::G711u,
            CodecFlag::G722 => polypaging::rtpcodec::CodecFlag::G722,
        }
    }
}

/// Gets the alert packet
#[pyfunction]
fn get_alert(session: SessionInfo) -> PyResult<Vec<u8>> {
    let real_session: polypaging::session::SessionInfo = session.get_real_session().unwrap();
    let result: polypaging::packet::PacketNoPayload = polypaging::packet::get_alert(&real_session);
    Ok(result.to_bytes().unwrap())
}

/// Gets the end packet
#[pyfunction]
fn get_end(session: SessionInfo) -> PyResult<Vec<u8>> {
    let real_session: polypaging::session::SessionInfo = session.get_real_session().unwrap();
    let result: polypaging::packet::PacketNoPayload = polypaging::packet::get_end(&real_session);
    Ok(result.to_bytes().unwrap())
}

/// Gets all of the payload packets for a single "file"
#[pyfunction]
fn get_payload_packets(session: SessionInfo, codec: CodecFlag, flags: u8, data: Vec<u8>) -> PyResult<Vec<Vec<u8>>> {
    let real_session: polypaging::session::SessionInfo = session.get_real_session().unwrap();
    let real_codec: polypaging::rtpcodec::CodecFlag = codec.to_codec_flag();
    let result: Vec<Vec<u8>> = polypaging::packet::get_payload_packets(&real_session, real_codec, flags, &data).unwrap();
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pypolypaging(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SessionInfo>()?;
    m.add_class::<CodecFlag>()?;
    m.add_function(wrap_pyfunction!(get_alert, m)?)?;
    m.add_function(wrap_pyfunction!(get_end, m)?)?;
    m.add_function(wrap_pyfunction!(get_payload_packets, m)?)?;
    Ok(())
}
