use super::*;

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

const BUS_SOCKET: &str = "/var/run/dbus/system_bus_socket";

const DBUS_NAME: &str = "org.freedesktop.DBus";
const DBUS_PATH: &str = "/org/freedesktop/DBus";
const OBJECT_MANAGER: &str = "org.freedesktop.DBus.ObjectManager";
const PROPERTIES_IFACE: &str = "org.freedesktop.DBus.Properties";
const BLUEZ_NAME: &str = "org.bluez";
const BLUEZ_ROOT: &str = "/";
const DEVICE_IFACE: &str = "org.bluez.Device1";
const GATT_CHAR_IFACE: &str = "org.bluez.GattCharacteristic1";

const TYPE_METHOD_CALL: u8 = 1;
const TYPE_METHOD_RETURN: u8 = 2;
const TYPE_DECLINE: u8 = 3;
const TYPE_SIGNAL: u8 = 4;

const FIELD_PATH: u8 = 1;
const FIELD_INTERFACE: u8 = 2;
const FIELD_MEMBER: u8 = 3;
const FIELD_DECLINE_NAME: u8 = 4;
const FIELD_REPLY_SERIAL: u8 = 5;
const FIELD_DESTINATION: u8 = 6;
const FIELD_SIGNATURE: u8 = 8;

const UUID_HR_MEASUREMENT: &str = "00002a37-0000-1000-8000-00805f9b34fb";

const GFDI_UUID_FRAGMENT: &str = "6a4e28";

const RR_UNIT_MS: f64 = 1000.0 / 1024.0;

const MAX_MESSAGE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, PartialEq)]
enum DbusValue {
    Byte(u8),
    Bool(bool),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F64(f64),
    Str(String),
    Bytes(Vec<u8>),
    Array(Vec<DbusValue>),
    Dict(Vec<(DbusValue, DbusValue)>),
    Struct(Vec<DbusValue>),
    Variant(Box<DbusValue>),
}

struct DbusMessage {
    msg_type: u8,
    reply_serial: Option<u32>,
    path: Option<String>,
    interface: Option<String>,
    member: Option<String>,
    decline_name: Option<String>,
    signature: String,
    args: Vec<DbusValue>,
}

enum Reply {
    Return(Vec<DbusValue>),
    Decline(Option<String>),
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Reader<'a> {
        Reader { buf, pos: 0 }
    }

    fn align(&mut self, a: usize) -> Option<()> {
        let target = (self.pos + a - 1) & !(a - 1);
        if target > self.buf.len() {
            return None;
        }
        self.pos = target;
        Some(())
    }

    fn byte(&mut self) -> Option<u8> {
        let b = *self.buf.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let s = self.buf.get(self.pos..end)?;
        self.pos = end;
        Some(s)
    }

    fn u32(&mut self) -> Option<u32> {
        let s = self.take(4)?;
        Some(u32::from_le_bytes(s.try_into().ok()?))
    }

    fn i16(&mut self) -> Option<i16> {
        let s = self.take(2)?;
        Some(i16::from_le_bytes(s.try_into().ok()?))
    }

    fn u16(&mut self) -> Option<u16> {
        let s = self.take(2)?;
        Some(u16::from_le_bytes(s.try_into().ok()?))
    }

    fn i32(&mut self) -> Option<i32> {
        let s = self.take(4)?;
        Some(i32::from_le_bytes(s.try_into().ok()?))
    }

    fn i64(&mut self) -> Option<i64> {
        let s = self.take(8)?;
        Some(i64::from_le_bytes(s.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        let s = self.take(8)?;
        Some(u64::from_le_bytes(s.try_into().ok()?))
    }

    fn f64(&mut self) -> Option<f64> {
        let s = self.take(8)?;
        Some(f64::from_le_bytes(s.try_into().ok()?))
    }

    fn str(&mut self) -> Option<String> {
        let n = self.u32()? as usize;
        let s = self.take(n)?;
        if self.byte()? != 0 {
            return None;
        }
        String::from_utf8(s.to_vec()).ok()
    }

    fn sig(&mut self) -> Option<String> {
        let n = self.byte()? as usize;
        let s = self.take(n)?;
        if self.byte()? != 0 {
            return None;
        }
        String::from_utf8(s.to_vec()).ok()
    }
}

struct Marshal {
    buf: Vec<u8>,
    base: usize,
}

impl Marshal {
    fn new(base: usize) -> Marshal {
        Marshal {
            buf: Vec::new(),
            base,
        }
    }

    fn align(&mut self, a: usize) {
        while !(self.base + self.buf.len()).is_multiple_of(a) {
            self.buf.push(0);
        }
    }

    fn raw(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    fn str(&mut self, s: &str) {
        self.align(4);
        self.buf.extend_from_slice(&(s.len() as u32).to_le_bytes());
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(0);
    }

    fn sig(&mut self, s: &str) {
        self.buf.push(s.len() as u8);
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(0);
    }

    fn field(&mut self, code: u8, sig: &str) {
        self.align(8);
        self.raw(&[code]);
        self.sig(sig);
    }
}

fn align_of(sig: &str) -> usize {
    match sig.as_bytes().first() {
        Some(b'y') | Some(b'g') => 1,
        Some(b'd') | Some(b'x') | Some(b't') | Some(b'(') | Some(b'{') => 8,
        _ => 4,
    }
}

fn complete_type(sig: &str) -> Option<(&str, &str)> {
    let first = sig.as_bytes().first()?;
    match first {
        b'a' => {
            let (inner, tail) = complete_type(&sig[1..])?;
            Some((&sig[..1 + inner.len()], tail))
        }
        b'(' | b'{' => {
            let close = if *first == b'(' { b')' } else { b'}' };
            let mut depth = 0usize;
            for (i, b) in sig.as_bytes().iter().enumerate() {
                if *b == *first {
                    depth += 1;
                } else if *b == close {
                    depth -= 1;
                    if depth == 0 {
                        return Some((&sig[..i + 1], &sig[i + 1..]));
                    }
                }
            }
            None
        }
        _ => Some((&sig[..1], &sig[1..])),
    }
}

fn unmarshal_struct(r: &mut Reader<'_>, inner: &str) -> Option<Vec<DbusValue>> {
    let mut out = Vec::new();
    let mut rest = inner;
    while !rest.is_empty() {
        let (complete, tail) = complete_type(rest)?;
        out.push(unmarshal_one(r, complete)?);
        rest = tail;
    }
    Some(out)
}

fn unmarshal_one(r: &mut Reader<'_>, sig: &str) -> Option<DbusValue> {
    match sig {
        "y" => r.byte().map(DbusValue::Byte),
        "b" => {
            r.align(4)?;
            Some(DbusValue::Bool(r.u32()? != 0))
        }
        "n" => {
            r.align(2)?;
            r.i16().map(DbusValue::I16)
        }
        "q" => {
            r.align(2)?;
            r.u16().map(DbusValue::U16)
        }
        "i" => {
            r.align(4)?;
            r.i32().map(DbusValue::I32)
        }
        "u" => {
            r.align(4)?;
            r.u32().map(DbusValue::U32)
        }
        "x" => {
            r.align(8)?;
            r.i64().map(DbusValue::I64)
        }
        "t" => {
            r.align(8)?;
            r.u64().map(DbusValue::U64)
        }
        "d" => {
            r.align(8)?;
            r.f64().map(DbusValue::F64)
        }
        "s" | "o" => {
            r.align(4)?;
            r.str().map(DbusValue::Str)
        }
        "g" => r.sig().map(DbusValue::Str),
        "v" => {
            let inner = r.sig()?;
            let value = unmarshal_one(r, &inner)?;
            Some(DbusValue::Variant(Box::new(value)))
        }
        "ay" => {
            r.align(4)?;
            let n = r.u32()? as usize;
            Some(DbusValue::Bytes(r.take(n)?.to_vec()))
        }
        _ if sig.starts_with("a{") => {
            r.align(4)?;
            let n = r.u32()? as usize;
            let inner = &sig[2..sig.len() - 1];
            r.align(8)?;
            let limit = r.pos.checked_add(n)?;
            if limit > r.buf.len() {
                return None;
            }
            let mut entries = Vec::new();
            while r.pos < limit {
                r.align(8)?;
                let (ktype, ktail) = complete_type(inner)?;
                let (vtype, vtail) = complete_type(ktail)?;
                if !vtail.is_empty() {
                    return None;
                }
                let k = unmarshal_one(r, ktype)?;
                let v = unmarshal_one(r, vtype)?;
                entries.push((k, v));
            }
            Some(DbusValue::Dict(entries))
        }
        _ if sig.starts_with("a(") => {
            r.align(4)?;
            let n = r.u32()? as usize;
            let inner = &sig[2..sig.len() - 1];
            r.align(8)?;
            let limit = r.pos.checked_add(n)?;
            if limit > r.buf.len() {
                return None;
            }
            let mut items = Vec::new();
            while r.pos < limit {
                r.align(8)?;
                items.push(DbusValue::Struct(unmarshal_struct(r, inner)?));
            }
            Some(DbusValue::Array(items))
        }
        _ if sig.starts_with('a') => {
            r.align(4)?;
            let n = r.u32()? as usize;
            let inner = &sig[1..];
            r.align(align_of(inner))?;
            let limit = r.pos.checked_add(n)?;
            if limit > r.buf.len() {
                return None;
            }
            let mut items = Vec::new();
            while r.pos < limit {
                items.push(unmarshal_one(r, inner)?);
            }
            Some(DbusValue::Array(items))
        }
        _ if sig.starts_with('(') => {
            r.align(8)?;
            let inner = &sig[1..sig.len() - 1];
            Some(DbusValue::Struct(unmarshal_struct(r, inner)?))
        }
        _ => None,
    }
}

fn unmarshal_body(sig: &str, body: &[u8]) -> Option<Vec<DbusValue>> {
    if sig.is_empty() || sig == "()" {
        return Some(Vec::new());
    }
    let mut r = Reader::new(body);
    let mut out = Vec::new();
    let mut rest = sig;
    while !rest.is_empty() {
        let (complete, tail) = complete_type(rest)?;
        out.push(unmarshal_one(&mut r, complete)?);
        rest = tail;
    }
    if r.pos != body.len() {
        return None;
    }
    Some(out)
}

fn marshal_message(serial: u32, msg_type: u8, mut fields: Marshal, body: &[u8]) -> Vec<u8> {
    let fields_len = fields.buf.len();
    while !(16 + fields.buf.len()).is_multiple_of(8) {
        fields.buf.push(0);
    }
    let mut msg = Vec::with_capacity(16 + fields.buf.len() + body.len());
    msg.push(0x6C);
    msg.push(msg_type);
    msg.push(0);
    msg.push(1);
    msg.extend_from_slice(&(body.len() as u32).to_le_bytes());
    msg.extend_from_slice(&serial.to_le_bytes());
    msg.extend_from_slice(&(fields_len as u32).to_le_bytes());
    msg.extend_from_slice(&fields.buf);
    msg.extend_from_slice(body);
    msg
}

fn marshal_method_call(
    serial: u32,
    destination: &str,
    path: &str,
    interface: &str,
    member: &str,
    body_sig: &str,
    body: &[u8],
) -> Vec<u8> {
    let mut fields = Marshal::new(16);
    fields.field(FIELD_PATH, "o");
    fields.str(path);
    fields.field(FIELD_INTERFACE, "s");
    fields.str(interface);
    fields.field(FIELD_MEMBER, "s");
    fields.str(member);
    fields.field(FIELD_DESTINATION, "s");
    fields.str(destination);
    if !body_sig.is_empty() {
        fields.field(FIELD_SIGNATURE, "g");
        fields.sig(body_sig);
    }
    marshal_message(serial, TYPE_METHOD_CALL, fields, body)
}

fn parse_message(bytes: &[u8]) -> Option<DbusMessage> {
    if bytes.len() < 16 || bytes[0] != 0x6C || bytes[3] != 1 {
        return None;
    }
    let body_len = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let fields_len = u32::from_le_bytes(bytes[12..16].try_into().ok()?) as usize;
    let fields_end = 16usize.checked_add(fields_len)?;
    let header_end = (fields_end + 7) & !7usize;
    let body_end = header_end.checked_add(body_len)?;
    if bytes.len() < body_end {
        return None;
    }
    let mut msg = DbusMessage {
        msg_type: bytes[1],
        reply_serial: None,
        path: None,
        interface: None,
        member: None,
        decline_name: None,
        signature: String::new(),
        args: Vec::new(),
    };
    let mut r = Reader::new(&bytes[16..fields_end]);
    while r.pos < r.buf.len() {
        r.align(8)?;
        let code = r.byte()?;
        let sig = r.sig()?;
        let value = unmarshal_one(&mut r, &sig)?;
        match code {
            FIELD_PATH => {
                if let DbusValue::Str(s) = value {
                    msg.path = Some(s);
                }
            }
            FIELD_INTERFACE => {
                if let DbusValue::Str(s) = value {
                    msg.interface = Some(s);
                }
            }
            FIELD_MEMBER => {
                if let DbusValue::Str(s) = value {
                    msg.member = Some(s);
                }
            }
            FIELD_DECLINE_NAME => {
                if let DbusValue::Str(s) = value {
                    msg.decline_name = Some(s);
                }
            }
            FIELD_REPLY_SERIAL => {
                if let DbusValue::U32(v) = value {
                    msg.reply_serial = Some(v);
                }
            }
            FIELD_SIGNATURE => {
                if let DbusValue::Str(s) = value {
                    msg.signature = s;
                }
            }
            _ => {}
        }
    }
    let body = &bytes[header_end..body_end];
    let args = unmarshal_body(&msg.signature, body)?;
    msg.args = args;
    Some(msg)
}

fn read_exact(stream: &mut UnixStream, buf: &mut [u8]) -> Option<()> {
    let mut filled = 0usize;
    while filled < buf.len() {
        match stream.read(&mut buf[filled..]) {
            Ok(0) => return None,
            Ok(n) => filled += n,
            Err(_) => return None,
        }
    }
    Some(())
}

fn read_auth_line(stream: &mut UnixStream, out: &mut Vec<u8>) -> Option<()> {
    out.clear();
    let mut buf = [0u8; 1];
    for _ in 0..4096 {
        match stream.read(&mut buf) {
            Ok(1) => {
                out.push(buf[0]);
                if out.ends_with(b"\r\n") {
                    return Some(());
                }
            }
            _ => return None,
        }
    }
    None
}

fn auth_line(uid: u32) -> Vec<u8> {
    let identity = uid.to_string();
    let mut hex = String::with_capacity(identity.len() * 2);
    for byte in identity.bytes() {
        hex.push_str(&format!("{byte:02x}"));
    }
    let mut line = Vec::new();
    line.push(0);
    line.extend_from_slice(format!("AUTH EXTERNAL {hex}\r\n").as_bytes());
    line
}

fn parse_status_uid(status: &str) -> Option<u32> {
    status
        .lines()
        .find_map(|line| line.strip_prefix("Uid:"))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|real| real.parse().ok())
}

fn process_uid() -> Option<u32> {
    parse_status_uid(&std::fs::read_to_string("/proc/self/status").ok()?)
}

struct SystemBus {
    stream: UnixStream,
    serial: u32,
}

impl SystemBus {
    fn open() -> Option<SystemBus> {
        let mut stream = UnixStream::connect(BUS_SOCKET).ok()?;
        let uid = process_uid()?;
        stream.write_all(&auth_line(uid)).ok()?;
        let mut line = Vec::new();
        read_auth_line(&mut stream, &mut line)?;
        if !line.starts_with(b"OK ") {
            return None;
        }
        stream.write_all(b"BEGIN\r\n").ok()?;
        Some(SystemBus { stream, serial: 1 })
    }

    fn next_message(&mut self) -> Option<DbusMessage> {
        let mut head = [0u8; 16];
        read_exact(&mut self.stream, &mut head)?;
        let body_len = u32::from_le_bytes(head[4..8].try_into().ok()?) as usize;
        let fields_len = u32::from_le_bytes(head[12..16].try_into().ok()?) as usize;
        let header_end = (16usize + fields_len + 7) & !7usize;
        let total = header_end.checked_add(body_len)?;
        if total > MAX_MESSAGE_BYTES {
            return None;
        }
        let mut bytes = vec![0u8; total];
        bytes[..16].copy_from_slice(&head);
        read_exact(&mut self.stream, &mut bytes[16..])?;
        parse_message(&bytes)
    }

    fn call(
        &mut self,
        destination: &str,
        path: &str,
        interface: &str,
        member: &str,
        body_sig: &str,
        body: &[u8],
    ) -> Option<Reply> {
        let sent = self.serial;
        let msg = marshal_method_call(sent, destination, path, interface, member, body_sig, body);
        self.serial = self.serial.wrapping_add(1);
        self.stream.write_all(&msg).ok()?;
        loop {
            let msg = self.next_message()?;
            match msg.msg_type {
                TYPE_METHOD_RETURN if msg.reply_serial == Some(sent) => {
                    return Some(Reply::Return(msg.args));
                }
                TYPE_DECLINE if msg.reply_serial == Some(sent) => {
                    return Some(Reply::Decline(msg.decline_name));
                }
                _ => {}
            }
        }
    }
}

fn variant_str(value: &DbusValue) -> Option<&str> {
    match value {
        DbusValue::Variant(inner) => match &**inner {
            DbusValue::Str(s) => Some(s),
            _ => None,
        },
        DbusValue::Str(s) => Some(s),
        _ => None,
    }
}

fn variant_bool(value: &DbusValue) -> Option<bool> {
    match value {
        DbusValue::Variant(inner) => match &**inner {
            DbusValue::Bool(b) => Some(*b),
            _ => None,
        },
        DbusValue::Bool(b) => Some(*b),
        _ => None,
    }
}

fn variant_bytes(value: &DbusValue) -> Option<&[u8]> {
    match value {
        DbusValue::Variant(inner) => match &**inner {
            DbusValue::Bytes(b) => Some(b),
            _ => None,
        },
        DbusValue::Bytes(b) => Some(b),
        _ => None,
    }
}

fn properties_changed<'a>(args: &'a [DbusValue], key: &str) -> Option<Option<&'a DbusValue>> {
    let changed = match args.get(1)? {
        DbusValue::Dict(entries) => entries,
        _ => return None,
    };
    for (name, value) in changed {
        if let DbusValue::Str(k) = name
            && k == key
        {
            return Some(Some(value));
        }
    }
    Some(None)
}

fn device_by_address(args: &[DbusValue], mac: &str) -> Option<(String, bool)> {
    let objects = match args.first()? {
        DbusValue::Dict(entries) => entries,
        _ => return None,
    };
    for (path, ifaces) in objects {
        let path = match path {
            DbusValue::Str(s) => s,
            _ => continue,
        };
        let ifaces = match ifaces {
            DbusValue::Dict(entries) => entries,
            _ => continue,
        };
        let mut address: Option<&str> = None;
        let mut connected: Option<bool> = None;
        for (name, props) in ifaces {
            let (name, props) = match (name, props) {
                (DbusValue::Str(n), DbusValue::Dict(p)) => (n, p),
                _ => continue,
            };
            if name != DEVICE_IFACE {
                continue;
            }
            for (key, value) in props {
                let key = match key {
                    DbusValue::Str(k) => k,
                    _ => continue,
                };
                match key.as_str() {
                    "Address" => {
                        address = variant_str(value);
                    }
                    "Connected" => {
                        connected = variant_bool(value);
                    }
                    _ => {}
                }
            }
        }
        if address
            .map(|a| a.eq_ignore_ascii_case(mac))
            .unwrap_or(false)
        {
            return Some((path.clone(), connected.unwrap_or(false)));
        }
    }
    None
}

fn characteristic_matches(
    args: &[DbusValue],
    device_path: &str,
    uuid_fragment: &str,
) -> Vec<(String, String)> {
    let fragment = uuid_fragment.to_ascii_lowercase();
    let mut matches = Vec::new();
    let Some(objects) = args.first().and_then(|a| match a {
        DbusValue::Dict(entries) => Some(entries),
        _ => None,
    }) else {
        return matches;
    };
    for (path, ifaces) in objects {
        let path = match path {
            DbusValue::Str(s) => s,
            _ => continue,
        };
        if !path.starts_with(device_path) {
            continue;
        }
        let ifaces = match ifaces {
            DbusValue::Dict(entries) => entries,
            _ => continue,
        };
        for (name, props) in ifaces {
            let (name, props) = match (name, props) {
                (DbusValue::Str(n), DbusValue::Dict(p)) => (n, p),
                _ => continue,
            };
            if name != GATT_CHAR_IFACE {
                continue;
            }
            for (key, value) in props {
                let key = match key {
                    DbusValue::Str(k) => k,
                    _ => continue,
                };
                if key == "UUID"
                    && let Some(uuid) = variant_str(value)
                    && uuid.to_ascii_lowercase().contains(&fragment)
                {
                    matches.push((path.clone(), uuid.to_string()));
                }
            }
        }
    }
    matches
}

fn characteristic_paths(args: &[DbusValue], device_path: &str, uuid_fragment: &str) -> Vec<String> {
    characteristic_matches(args, device_path, uuid_fragment)
        .into_iter()
        .map(|(path, _)| path)
        .collect()
}

fn hr_measurement_path(args: &[DbusValue], device_path: &str) -> Option<String> {
    characteristic_paths(args, device_path, UUID_HR_MEASUREMENT)
        .into_iter()
        .next()
}

fn hex_bytes(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

fn gfdi_line(uuid: &str, payload: &[u8]) -> String {
    format!("ble gfdi {uuid} {}", hex_bytes(payload))
}

fn plausible(v: f64) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

const GFDI_FRAME_MIN: usize = 6;

const GFDI_TYPE_BASE: u16 = 5000;
const GFDI_TYPE_PROTOBUF_REQUEST: u16 = 5043;
const GFDI_TYPE_PROTOBUF_RESPONSE: u16 = 5044;

const GFDI_PROTO_HEADER: usize = 14;

const WIRE_VARINT: u8 = 0;
const WIRE_FIXED64: u8 = 1;
const WIRE_LEN: u8 = 2;
const WIRE_FIXED32: u8 = 5;

const FS_SMART_FILE_SYNC: u32 = 43;
const FS_FILE_REQUEST: u32 = 1;
const FS_FILE_RESPONSE: u32 = 2;
const FS_FILE_LIST_REQUEST: u32 = 9;
const FS_FILE_LIST_RESPONSE: u32 = 10;
const FS_NEW_FILE_NOTIFICATION: u32 = 12;
const FS_FILE_SET_FLAGS: u32 = 15;
const FS_FILE_UPDATE_NOTIFICATION: u32 = 17;

fn crc16_arc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &b in data {
        crc ^= u16::from(b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xA001
            } else {
                crc >> 1
            };
        }
    }
    crc
}

struct GfdiFrame<'a> {
    msg_type: u16,
    payload: &'a [u8],
}

fn split_gfdi_frames(stream: &[u8]) -> Vec<GfdiFrame<'_>> {
    let mut frames = Vec::new();
    let mut pos = 0usize;
    while pos < stream.len() {
        let rest = &stream[pos..];
        if rest.len() < GFDI_FRAME_MIN {
            break;
        }
        let size = usize::from(u16::from_le_bytes([rest[0], rest[1]]));
        if size < GFDI_FRAME_MIN || size > rest.len() {
            pos += 1;
            continue;
        }
        let frame = &rest[..size];
        let stored = u16::from_le_bytes([frame[size - 2], frame[size - 1]]);
        if crc16_arc(&frame[..size - 2]) != stored {
            pos += 1;
            continue;
        }
        let raw_type = u16::from_le_bytes([frame[2], frame[3]]);
        let msg_type = if raw_type & 0x8000 != 0 {
            (raw_type & 0x00FF) + GFDI_TYPE_BASE
        } else {
            raw_type
        };
        frames.push(GfdiFrame {
            msg_type,
            payload: &frame[4..size - 2],
        });
        pos += size;
    }
    frames
}

fn protobuf_declared_len(payload: &[u8]) -> Option<usize> {
    if payload.len() < GFDI_PROTO_HEADER {
        return None;
    }
    usize::try_from(u32::from_le_bytes(payload[10..14].try_into().ok()?)).ok()
}

fn protobuf_payload(payload: &[u8]) -> Option<&[u8]> {
    let declared = protobuf_declared_len(payload)?;
    payload.get(GFDI_PROTO_HEADER..)?.get(..declared)
}

struct GfdiReassembler {
    declared: Option<usize>,
    body: Vec<u8>,
}

impl GfdiReassembler {
    fn new() -> GfdiReassembler {
        GfdiReassembler {
            declared: None,
            body: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.declared = None;
        self.body.clear();
    }

    fn push(&mut self, payload: &[u8]) -> Option<Vec<u8>> {
        match self.declared {
            None => {
                if let Some(proto) = protobuf_payload(payload) {
                    return Some(proto.to_vec());
                }
                let declared = protobuf_declared_len(payload)?;
                self.declared = Some(declared);
                self.body
                    .extend_from_slice(payload.get(GFDI_PROTO_HEADER..)?);
                None
            }
            Some(declared) => {
                self.body
                    .extend_from_slice(payload.get(GFDI_PROTO_HEADER..)?);
                if self.body.len() < declared {
                    return None;
                }
                let complete = self.body[..declared].to_vec();
                self.reset();
                Some(complete)
            }
        }
    }
}

struct ProtoReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ProtoReader<'a> {
    fn new(buf: &'a [u8]) -> ProtoReader<'a> {
        ProtoReader { buf, pos: 0 }
    }

    fn varint(&mut self) -> Option<u64> {
        let mut value: u64 = 0;
        for i in 0..10 {
            let b = *self.buf.get(self.pos)?;
            self.pos += 1;
            if i == 9 && b & 0xFE != 0 {
                return None;
            }
            value |= u64::from(b & 0x7F) << (i * 7);
            if b & 0x80 == 0 {
                return Some(value);
            }
        }
        None
    }

    fn fixed64(&mut self) -> Option<u64> {
        let s = self.take(8)?;
        Some(u64::from_le_bytes(s.try_into().ok()?))
    }

    fn fixed32(&mut self) -> Option<u64> {
        let s = self.take(4)?;
        Some(u64::from(u32::from_le_bytes(s.try_into().ok()?)))
    }

    fn len_delimited(&mut self) -> Option<&'a [u8]> {
        let n = self.varint()?;
        self.take(usize::try_from(n).ok()?)
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let s = self.buf.get(self.pos..end)?;
        self.pos = end;
        Some(s)
    }

    fn skip_field(&mut self, wire: u8) -> bool {
        match wire {
            WIRE_VARINT => self.varint().is_some(),
            WIRE_FIXED64 => self.fixed64().is_some(),
            WIRE_LEN => self.len_delimited().is_some(),
            WIRE_FIXED32 => self.fixed32().is_some(),
            _ => false,
        }
    }

    fn next_field(&mut self) -> Option<(u32, u8)> {
        let tag = self.varint()?;
        let field = tag >> 3;
        if field == 0 || field > u64::from(u32::MAX) {
            return None;
        }
        Some((field as u32, (tag & 0x07) as u8))
    }
}

fn push_record(out: &mut Vec<(String, f64, Option<f64>)>, name: &str, raw: u64) {
    if let Some(v) = plausible(raw as f64) {
        out.push((name.to_string(), v, None));
    }
}

fn gfdi_records(payload: &[u8]) -> Vec<(String, f64, Option<f64>)> {
    let mut out = Vec::new();
    let mut reassembler = GfdiReassembler::new();
    for frame in split_gfdi_frames(payload) {
        if frame.msg_type != GFDI_TYPE_PROTOBUF_REQUEST
            && frame.msg_type != GFDI_TYPE_PROTOBUF_RESPONSE
        {
            reassembler.reset();
            continue;
        }
        let Some(proto) = reassembler.push(frame.payload) else {
            continue;
        };
        filesync_records(&proto, &mut out);
    }
    out
}

fn filesync_records(proto: &[u8], out: &mut Vec<(String, f64, Option<f64>)>) {
    let mut r = ProtoReader::new(proto);
    while let Some((field, wire)) = r.next_field() {
        if field == FS_SMART_FILE_SYNC
            && wire == WIRE_LEN
            && let Some(sub) = r.len_delimited()
        {
            filesync_service(out, sub);
            continue;
        }
        if !r.skip_field(wire) {
            break;
        }
    }
}

fn filesync_service(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (FS_FILE_REQUEST, WIRE_LEN) | (FS_NEW_FILE_NOTIFICATION, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_carrier(out, sub);
            }
            (FS_FILE_RESPONSE, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_response(out, sub);
            }
            (FS_FILE_LIST_REQUEST, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_list_request(out, sub);
            }
            (FS_FILE_LIST_RESPONSE, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_list_response(out, sub);
            }
            (FS_FILE_SET_FLAGS, WIRE_LEN) | (FS_FILE_UPDATE_NOTIFICATION, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_id_carrier(out, sub);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_carrier(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (1, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file(out, sub);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_id_carrier(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (1, WIRE_LEN) | (2, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_id(out, sub);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (1, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_id(out, sub);
            }
            (2, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file_type(out, sub);
            }
            (3, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.file.size", raw);
            }
            (5, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.file.page_id", raw);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_type(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (3, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.file.type.code", raw);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_id(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (1, WIRE_FIXED64) => {
                let Some(raw) = r.fixed64() else { return };
                push_record(out, "gfdi.file.id1", raw);
            }
            (2, WIRE_FIXED64) => {
                let Some(raw) = r.fixed64() else { return };
                push_record(out, "gfdi.file.id2", raw);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_response(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (1, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.response.status", raw);
            }
            (3, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.response.handle", raw);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_list_request(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (2, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.list_request.start_page_id", raw);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

fn file_list_response(out: &mut Vec<(String, f64, Option<f64>)>, bytes: &[u8]) {
    let mut r = ProtoReader::new(bytes);
    while let Some((field, wire)) = r.next_field() {
        match (field, wire) {
            (3, WIRE_VARINT) => {
                let Some(raw) = r.varint() else { return };
                push_record(out, "gfdi.list_response.next_page_id", raw);
            }
            (4, WIRE_LEN) => {
                let Some(sub) = r.len_delimited() else { return };
                file(out, sub);
            }
            _ => {
                if !r.skip_field(wire) {
                    return;
                }
            }
        }
    }
}

pub fn decode_hr_measurement(payload: &[u8]) -> Option<(Option<f64>, Vec<f64>)> {
    let (flags, rest) = payload.split_first()?;
    let hr16 = flags & 0x01 != 0;
    let energy = flags & 0x08 != 0;
    let rr = flags & 0x10 != 0;
    let mut pos = 0usize;
    let hr = if hr16 {
        let raw = u16::from_le_bytes([*rest.get(pos)?, *rest.get(pos + 1)?]);
        pos += 2;
        plausible(raw as f64)
    } else {
        let raw = *rest.get(pos)? as f64;
        pos += 1;
        plausible(raw)
    };
    if energy {
        rest.get(pos..pos + 2)?;
        pos += 2;
    }
    let mut intervals = Vec::new();
    if rr {
        while pos < rest.len() {
            if rest.len() - pos < 2 {
                return None;
            }
            let raw = u16::from_le_bytes([rest[pos], rest[pos + 1]]);
            pos += 2;
            if raw == 0 {
                continue;
            }
            let ms = raw as f64 * RR_UNIT_MS;
            if ms.is_finite() && ms > 0.0 {
                intervals.push(ms);
            }
        }
    }
    Some((hr, intervals))
}

fn ble_session(tx: &mpsc::Sender<Vec<(String, f64, Option<f64>)>>, mac: &str) {
    let mut bus = match SystemBus::open() {
        Some(b) => b,
        None => {
            eprintln!("ble: system bus socket unreachable — the beat source stays silent");
            return;
        }
    };
    if !matches!(
        bus.call(DBUS_NAME, DBUS_PATH, DBUS_NAME, "Hello", "", &[]),
        Some(Reply::Return(_))
    ) {
        eprintln!("ble: Hello void — the beat source stays silent");
        return;
    }
    let mut body = Marshal::new(0);
    body.str("type='signal',sender='org.bluez'");
    if !matches!(
        bus.call(DBUS_NAME, DBUS_PATH, DBUS_NAME, "AddMatch", "s", &body.buf),
        Some(Reply::Return(_))
    ) {
        eprintln!("ble: AddMatch void — the beat source stays silent");
        return;
    }
    let objects = match bus.call(
        BLUEZ_NAME,
        BLUEZ_ROOT,
        OBJECT_MANAGER,
        "GetManagedObjects",
        "",
        &[],
    ) {
        Some(Reply::Return(args)) => args,
        _ => {
            eprintln!("ble: GetManagedObjects void — the beat source stays silent");
            return;
        }
    };
    let Some((device_path, connected)) = device_by_address(&objects, mac) else {
        eprintln!("ble: no BlueZ device carries address {mac} — the beat source stays silent");
        return;
    };
    if !connected {
        match bus.call(BLUEZ_NAME, &device_path, DEVICE_IFACE, "Connect", "", &[]) {
            Some(Reply::Return(_)) => {}
            Some(Reply::Decline(name)) => {
                eprintln!(
                    "ble: Connect declined ({}) — the beat source stays silent",
                    name.as_deref().unwrap_or("unnamed")
                );
                return;
            }
            None => {
                eprintln!("ble: Connect reply void — the beat source stays silent");
                return;
            }
        }
    }
    let objects = match bus.call(
        BLUEZ_NAME,
        BLUEZ_ROOT,
        OBJECT_MANAGER,
        "GetManagedObjects",
        "",
        &[],
    ) {
        Some(Reply::Return(args)) => args,
        _ => {
            eprintln!("ble: GetManagedObjects void — the beat source stays silent");
            return;
        }
    };
    let Some(char_path) = hr_measurement_path(&objects, &device_path) else {
        eprintln!(
            "ble: the HR measurement characteristic (0x2a37) is unresolved on {device_path} — retry in the next pass"
        );
        return;
    };
    match bus.call(
        BLUEZ_NAME,
        &char_path,
        GATT_CHAR_IFACE,
        "StartNotify",
        "",
        &[],
    ) {
        Some(Reply::Return(_)) => {}
        Some(Reply::Decline(Some(name))) if name.ends_with(".InProgress") => {}
        Some(Reply::Decline(name)) => {
            eprintln!(
                "ble: StartNotify declined ({}) — the beat source stays silent",
                name.as_deref().unwrap_or("unnamed")
            );
            return;
        }
        None => {
            eprintln!("ble: StartNotify reply void — the beat source stays silent");
            return;
        }
    }
    let gfdi_chars = characteristic_matches(&objects, &device_path, GFDI_UUID_FRAGMENT);
    for (uuid, path) in &gfdi_chars {
        match bus.call(BLUEZ_NAME, path, GATT_CHAR_IFACE, "StartNotify", "", &[]) {
            Some(Reply::Return(_)) => {}
            Some(Reply::Decline(Some(name))) if name.ends_with(".InProgress") => {}
            Some(Reply::Decline(name)) => {
                eprintln!(
                    "ble: GFDI StartNotify on {uuid} declined ({}) — the raw transport stays silent for this characteristic",
                    name.as_deref().unwrap_or("unnamed")
                );
            }
            None => {
                eprintln!(
                    "ble: GFDI StartNotify on {uuid} reply void — the raw transport stays silent for this characteristic"
                );
            }
        }
    }
    loop {
        let Some(msg) = bus.next_message() else {
            eprintln!("ble: the system bus connection closed — reconnecting");
            return;
        };
        if msg.msg_type != TYPE_SIGNAL
            || msg.interface.as_deref() != Some(PROPERTIES_IFACE)
            || msg.member.as_deref() != Some("PropertiesChanged")
        {
            continue;
        }
        if msg.path.as_deref() == Some(device_path.as_str()) {
            if let Some(Some(value)) = properties_changed(&msg.args, "Connected")
                && variant_bool(value) == Some(false)
            {
                eprintln!("ble: the device disconnected — reconnecting");
                return;
            }
            continue;
        }
        if msg.path.as_deref() == Some(char_path.as_str()) {
            let Some(value) = properties_changed(&msg.args, "Value").flatten() else {
                continue;
            };
            let Some(payload) = variant_bytes(value) else {
                continue;
            };
            let Some((_, intervals)) = decode_hr_measurement(payload) else {
                continue;
            };
            if intervals.is_empty() {
                continue;
            }
            let batch: Vec<(String, f64, Option<f64>)> = intervals
                .into_iter()
                .map(|ms| ("rr".to_string(), ms, None))
                .collect();
            let _ = tx.send(batch);
            continue;
        }
        if let Some(path) = msg.path.as_deref()
            && let Some((uuid, _)) = gfdi_chars.iter().find(|(_, p)| p == path)
        {
            let Some(value) = properties_changed(&msg.args, "Value").flatten() else {
                continue;
            };
            let Some(payload) = variant_bytes(value) else {
                continue;
            };
            eprintln!("{}", gfdi_line(uuid, payload));
            let batch = gfdi_records(payload);
            if !batch.is_empty() {
                let _ = tx.send(batch);
            }
        }
    }
}

pub fn ble_ingress(tx: mpsc::Sender<Vec<(String, f64, Option<f64>)>>) {
    let mac = match std::env::var("OMEGAFLOW_BLE_HR") {
        Ok(v) if !v.trim().is_empty() => v.trim().to_string(),
        _ => return,
    };
    if std::env::var("OMEGAFLOW_SERIAL_IN").is_ok() {
        eprintln!(
            "beat sources conflict: OMEGAFLOW_BLE_HR and OMEGAFLOW_SERIAL_IN are both set — one beat source at a time; the BLE source stays silent"
        );
        return;
    }
    loop {
        ble_session(&tx, &mac);
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIELD_SENDER: u8 = 7;

    fn le32(bytes: &[u8]) -> u32 {
        u32::from_le_bytes(bytes.try_into().expect("four bytes"))
    }

    #[test]
    fn method_call_frames_like_the_wire() {
        let msg = marshal_method_call(
            1,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
            "Hello",
            "",
            &[],
        );
        assert_eq!(&msg[..4], &[0x6C, 1, 0, 1]);
        assert_eq!(le32(&msg[4..8]), 0);
        assert_eq!(le32(&msg[8..12]), 1);
        let fields_len = le32(&msg[12..16]) as usize;
        let header_end = (16 + fields_len + 7) & !7;
        assert_eq!(msg.len(), header_end);
        let parsed = parse_message(&msg).expect("valid message");
        assert_eq!(parsed.msg_type, TYPE_METHOD_CALL);
        assert_eq!(parsed.member.as_deref(), Some("Hello"));
        assert_eq!(parsed.path.as_deref(), Some("/org/freedesktop/DBus"));
        assert!(parsed.args.is_empty());
    }

    #[test]
    fn method_return_carries_reply_serial_and_body() {
        let mut fields = Marshal::new(16);
        fields.field(FIELD_REPLY_SERIAL, "u");
        fields.align(4);
        fields.buf.extend_from_slice(&1u32.to_le_bytes());
        fields.field(FIELD_SIGNATURE, "g");
        fields.sig("s");
        let mut body = Marshal::new(0);
        body.str(":1.42");
        let msg = marshal_message(42, TYPE_METHOD_RETURN, fields, &body.buf);
        let parsed = parse_message(&msg).expect("valid reply");
        assert_eq!(parsed.msg_type, TYPE_METHOD_RETURN);
        assert_eq!(parsed.reply_serial, Some(1));
        assert_eq!(parsed.signature, "s");
        assert_eq!(parsed.args.len(), 1);
        assert!(matches!(&parsed.args[0], DbusValue::Str(s) if s == ":1.42"));
    }

    #[test]
    fn decline_message_parses_decline_name() {
        let mut fields = Marshal::new(16);
        fields.field(FIELD_REPLY_SERIAL, "u");
        fields.align(4);
        fields.buf.extend_from_slice(&1u32.to_le_bytes());
        fields.field(FIELD_DECLINE_NAME, "s");
        fields.str("org.bluez.NotReady");
        let msg = marshal_message(2, TYPE_DECLINE, fields, &[]);
        let parsed = parse_message(&msg).expect("valid decline");
        assert_eq!(parsed.msg_type, TYPE_DECLINE);
        assert_eq!(parsed.decline_name.as_deref(), Some("org.bluez.NotReady"));
        assert_eq!(parsed.reply_serial, Some(1));
    }

    #[test]
    fn signal_properties_changed_parses_value_bytes() {
        let mut fields = Marshal::new(16);
        fields.field(FIELD_PATH, "o");
        fields.str("/org/bluez/hci0/dev_F0_99_19_4E_0B_BF/service0010/char0015");
        fields.field(FIELD_INTERFACE, "s");
        fields.str(PROPERTIES_IFACE);
        fields.field(FIELD_MEMBER, "s");
        fields.str("PropertiesChanged");
        fields.field(FIELD_SENDER, "s");
        fields.str(BLUEZ_NAME);
        fields.field(FIELD_SIGNATURE, "g");
        fields.sig("sa{sv}as");
        let mut body = Marshal::new(0);
        body.align(8);
        body.str("org.bluez.GattCharacteristic1");
        body.align(4);
        let dict_pos = body.buf.len();
        body.buf.extend_from_slice(&[0, 0, 0, 0]);
        body.align(8);
        let dict_elems = body.buf.len();
        body.str("Value");
        body.buf.extend_from_slice(&[2, b'a', b'y', 0]);
        body.align(4);
        body.buf.extend_from_slice(&4u32.to_le_bytes());
        body.buf.extend_from_slice(&[0x10, 0x3C, 0x08, 0x00]);
        let dict_len = (body.buf.len() - dict_elems) as u32;
        body.buf[dict_pos..dict_pos + 4].copy_from_slice(&dict_len.to_le_bytes());
        body.align(4);
        body.buf.extend_from_slice(&0u32.to_le_bytes());
        let msg = marshal_message(7, TYPE_SIGNAL, fields, &body.buf);
        let parsed = parse_message(&msg).expect("valid signal");
        assert_eq!(parsed.msg_type, TYPE_SIGNAL);
        assert_eq!(
            parsed.path.as_deref(),
            Some("/org/bluez/hci0/dev_F0_99_19_4E_0B_BF/service0010/char0015")
        );
        assert_eq!(parsed.member.as_deref(), Some("PropertiesChanged"));
        let value = properties_changed(&parsed.args, "Value")
            .expect("valid properties")
            .expect("value present");
        let payload = variant_bytes(value).expect("byte array value");
        assert_eq!(payload, &[0x10, 0x3C, 0x08, 0x00]);
        let (_, rr) = decode_hr_measurement(payload).expect("valid measurement");
        assert_eq!(rr, vec![8.0 * 1000.0 / 1024.0]);
    }

    #[test]
    fn managed_objects_reply_resolves_device_and_characteristic() {
        let mut body = Marshal::new(0);
        body.align(4);
        let top = body.buf.len();
        body.buf.extend_from_slice(&[0, 0, 0, 0]);
        body.align(8);
        let top_elems = body.buf.len();
        body.str("/org/bluez/hci0/dev_F0_99_19_4E_0B_BF");
        body.align(4);
        let ifs = body.buf.len();
        body.buf.extend_from_slice(&[0, 0, 0, 0]);
        body.align(8);
        let ifs_elems = body.buf.len();
        body.str(DEVICE_IFACE);
        body.align(4);
        let props = body.buf.len();
        body.buf.extend_from_slice(&[0, 0, 0, 0]);
        body.align(8);
        let props_elems = body.buf.len();
        body.str("Address");
        body.buf.extend_from_slice(&[1, b's', 0]);
        body.str("F0:99:19:4E:0B:BF");
        body.align(8);
        body.str("Connected");
        body.buf.extend_from_slice(&[1, b'b', 0]);
        body.align(4);
        body.buf.extend_from_slice(&1u32.to_le_bytes());
        let props_len = (body.buf.len() - props_elems) as u32;
        body.buf[props..props + 4].copy_from_slice(&props_len.to_le_bytes());
        let ifs_len = (body.buf.len() - ifs_elems) as u32;
        body.buf[ifs..ifs + 4].copy_from_slice(&ifs_len.to_le_bytes());
        body.align(8);
        body.str("/org/bluez/hci0/dev_F0_99_19_4E_0B_BF/service0010/char0015");
        body.align(4);
        let ifs2 = body.buf.len();
        body.buf.extend_from_slice(&[0, 0, 0, 0]);
        body.align(8);
        let ifs2_elems = body.buf.len();
        body.str(GATT_CHAR_IFACE);
        body.align(4);
        let props2 = body.buf.len();
        body.buf.extend_from_slice(&[0, 0, 0, 0]);
        body.align(8);
        let props2_elems = body.buf.len();
        body.str("UUID");
        body.buf.extend_from_slice(&[1, b's', 0]);
        body.str(UUID_HR_MEASUREMENT);
        let props2_len = (body.buf.len() - props2_elems) as u32;
        body.buf[props2..props2 + 4].copy_from_slice(&props2_len.to_le_bytes());
        let ifs2_len = (body.buf.len() - ifs2_elems) as u32;
        body.buf[ifs2..ifs2 + 4].copy_from_slice(&ifs2_len.to_le_bytes());
        let top_len = (body.buf.len() - top_elems) as u32;
        body.buf[top..top + 4].copy_from_slice(&top_len.to_le_bytes());

        let mut fields = Marshal::new(16);
        fields.field(FIELD_REPLY_SERIAL, "u");
        fields.align(4);
        fields.buf.extend_from_slice(&3u32.to_le_bytes());
        fields.field(FIELD_SIGNATURE, "g");
        fields.sig("a{oa{sa{sv}}}");
        let msg = marshal_message(9, TYPE_METHOD_RETURN, fields, &body.buf);
        let parsed = parse_message(&msg).expect("valid reply");
        let (path, connected) =
            device_by_address(&parsed.args, "f0:99:19:4e:0b:bf").expect("device resolved");
        assert_eq!(path, "/org/bluez/hci0/dev_F0_99_19_4E_0B_BF");
        assert!(connected);
        let char_path = hr_measurement_path(&parsed.args, &path).expect("characteristic resolved");
        assert_eq!(
            char_path,
            "/org/bluez/hci0/dev_F0_99_19_4E_0B_BF/service0010/char0015"
        );
    }

    #[test]
    fn complete_type_splits_nested_dict_signatures() {
        assert_eq!(complete_type("a{oa{sa{sv}}}"), Some(("a{oa{sa{sv}}}", "")));
        assert_eq!(complete_type("sa{sv}as"), Some(("s", "a{sv}as")));
        assert_eq!(complete_type("(sa{sv}as)"), Some(("(sa{sv}as)", "")));
        assert_eq!(complete_type("ay"), Some(("ay", "")));
    }

    #[test]
    fn auth_line_encodes_the_uid_as_hex() {
        assert_eq!(auth_line(1000), b"\0AUTH EXTERNAL 31303030\r\n");
        assert_eq!(auth_line(0), b"\0AUTH EXTERNAL 30\r\n");
    }

    #[test]
    fn parse_status_uid_reads_the_real_uid() {
        assert_eq!(
            parse_status_uid("Name:\tcat\nUid:\t1000\t1000\t1000\t1000\n"),
            Some(1000)
        );
        assert_eq!(parse_status_uid("Name:\tcat\nGid:\t1000\n"), None);
    }

    #[test]
    fn hr_measurement_8bit_with_rr_decodes() {
        let payload = [0x10, 0x3C, 0x08, 0x00];
        let (hr, rr) = decode_hr_measurement(&payload).expect("valid measurement");
        assert_eq!(hr, Some(60.0));
        assert_eq!(rr, vec![8.0 * 1000.0 / 1024.0]);
    }

    #[test]
    fn hr_measurement_16bit_decodes() {
        let payload = [0x01, 0x34, 0x00];
        let (hr, rr) = decode_hr_measurement(&payload).expect("valid measurement");
        assert_eq!(hr, Some(52.0));
        assert!(rr.is_empty());
    }

    #[test]
    fn hr_measurement_skips_energy_field() {
        let payload = [0x18, 0x3C, 0xE8, 0x03, 0x08, 0x00];
        let (hr, rr) = decode_hr_measurement(&payload).expect("valid measurement");
        assert_eq!(hr, Some(60.0));
        assert_eq!(rr, vec![8.0 * 1000.0 / 1024.0]);
    }

    #[test]
    fn hr_measurement_rr_zero_is_absent() {
        let payload = [0x10, 0x3C, 0x00, 0x00, 0x08, 0x00];
        let (_, rr) = decode_hr_measurement(&payload).expect("valid measurement");
        assert_eq!(rr, vec![8.0 * 1000.0 / 1024.0]);
    }

    #[test]
    fn hr_measurement_truncated_payload_is_absent() {
        assert_eq!(decode_hr_measurement(&[0x10, 0x3C, 0x08]), None);
        assert_eq!(decode_hr_measurement(&[]), None);
    }

    #[test]
    fn hr_measurement_zero_hr_is_absent() {
        let payload = [0x10, 0x00, 0x08, 0x00];
        let (hr, rr) = decode_hr_measurement(&payload).expect("valid measurement");
        assert_eq!(hr, None);
        assert_eq!(rr, vec![8.0 * 1000.0 / 1024.0]);
    }

    #[test]
    fn hr_measurement_no_rr_flag_leaves_intervals_empty() {
        let payload = [0x00, 0x3C];
        let (hr, rr) = decode_hr_measurement(&payload).expect("valid measurement");
        assert_eq!(hr, Some(60.0));
        assert!(rr.is_empty());
    }

    fn managed_characteristic(path: &str, uuid: &str) -> (DbusValue, DbusValue) {
        (
            DbusValue::Str(path.to_string()),
            DbusValue::Dict(vec![(
                DbusValue::Str(GATT_CHAR_IFACE.to_string()),
                DbusValue::Dict(vec![(
                    DbusValue::Str("UUID".to_string()),
                    DbusValue::Variant(Box::new(DbusValue::Str(uuid.to_string()))),
                )]),
            )]),
        )
    }

    #[test]
    fn characteristic_paths_match_uuid_fragment() {
        let gfdi_uuid = "test-6a4e28-char";
        let args = vec![DbusValue::Dict(vec![
            managed_characteristic("/org/bluez/hci0/dev_X/service0/char_gfdi", gfdi_uuid),
            managed_characteristic(
                "/org/bluez/hci0/dev_X/service0/char_hr",
                UUID_HR_MEASUREMENT,
            ),
        ])];
        let device = "/org/bluez/hci0/dev_X";
        assert_eq!(
            characteristic_paths(&args, device, GFDI_UUID_FRAGMENT),
            vec!["/org/bluez/hci0/dev_X/service0/char_gfdi".to_string()]
        );
        assert_eq!(
            characteristic_paths(&args, device, "2a37"),
            vec!["/org/bluez/hci0/dev_X/service0/char_hr".to_string()]
        );
    }

    #[test]
    fn gfdi_line_prints_lowercase_hex() {
        assert_eq!(
            gfdi_line("test-6a4e28-char", &[0x0a, 0x00, 0xff, 0x10, 0xa5]),
            "ble gfdi test-6a4e28-char 0a00ff10a5"
        );
    }

    fn gfdi_frame(msg_type: u16, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::with_capacity(payload.len() + GFDI_FRAME_MIN);
        frame.extend_from_slice(&((payload.len() + GFDI_FRAME_MIN) as u16).to_le_bytes());
        frame.extend_from_slice(&msg_type.to_le_bytes());
        frame.extend_from_slice(payload);
        let crc = crc16_arc(&frame);
        frame.extend_from_slice(&crc.to_le_bytes());
        frame
    }

    fn protobuf_frame(msg_type: u16, proto: &[u8]) -> Vec<u8> {
        let mut payload = Vec::with_capacity(GFDI_PROTO_HEADER + proto.len());
        payload.extend_from_slice(&0x2Au16.to_le_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&(proto.len() as u32).to_le_bytes());
        payload.extend_from_slice(&(proto.len() as u32).to_le_bytes());
        payload.extend_from_slice(proto);
        gfdi_frame(msg_type, &payload)
    }

    fn protobuf_chunk(msg_type: u16, declared: usize, body: &[u8]) -> Vec<u8> {
        let mut payload = Vec::with_capacity(GFDI_PROTO_HEADER + body.len());
        payload.extend_from_slice(&0x2Au16.to_le_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&(declared as u32).to_le_bytes());
        payload.extend_from_slice(&(declared as u32).to_le_bytes());
        payload.extend_from_slice(body);
        gfdi_frame(msg_type, &payload)
    }

    const FILE_LIST_RESPONSE_PROTO: [u8; 18] = [
        0xDA, 0x02, 0x0F, 0x52, 0x0D, 0x18, 0xC9, 0x87, 0x02, 0x22, 0x07, 0x18, 0xB4, 0x24, 0x28,
        0xF4, 0x86, 0x02,
    ];

    #[test]
    fn crc16_arc_matches_the_reference_vectors() {
        assert_eq!(crc16_arc(b"123456789"), 0xBB3D);
        assert_eq!(crc16_arc(b""), 0x0000);
    }

    #[test]
    fn split_gfdi_frames_extracts_reference_frames_from_the_zig_source() {
        let ack = [0x09, 0x00, 0x88, 0x13, 0xA0, 0x13, 0x00, 0x70, 0x89];
        let event = [0x08, 0x00, 0xA6, 0x13, 0x08, 0x00, 0xD5, 0xC5];
        let types_req = [0x06, 0x00, 0xA7, 0x13, 0x3B, 0x75];
        let mut stream = Vec::new();
        stream.extend_from_slice(&ack);
        stream.extend_from_slice(&event);
        stream.extend_from_slice(&types_req);
        let frames = split_gfdi_frames(&stream);
        assert_eq!(frames.len(), 3);
        assert_eq!(frames[0].msg_type, 5000);
        assert_eq!(frames[0].payload, &[0xA0, 0x13, 0x00]);
        assert_eq!(frames[1].msg_type, 5030);
        assert_eq!(frames[1].payload, &[0x08, 0x00]);
        assert_eq!(frames[2].msg_type, 5031);
        assert!(frames[2].payload.is_empty());
    }

    #[test]
    fn split_gfdi_frames_skips_a_frame_with_a_corrupted_crc() {
        let mut corrupt = [0x09, 0x00, 0x88, 0x13, 0xA0, 0x13, 0x00, 0x70, 0x89];
        corrupt[8] ^= 0xFF;
        assert!(split_gfdi_frames(&corrupt).is_empty());
        let event = [0x08, 0x00, 0xA6, 0x13, 0x08, 0x00, 0xD5, 0xC5];
        let mut stream = Vec::new();
        stream.extend_from_slice(&corrupt);
        stream.extend_from_slice(&event);
        let frames = split_gfdi_frames(&stream);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].msg_type, 5030);
    }

    #[test]
    fn split_gfdi_frames_decodes_the_short_type_form() {
        let frame = gfdi_frame(0x822B, &[0xAA, 0xBB]);
        let frames = split_gfdi_frames(&frame);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].msg_type, 5043);
        assert_eq!(frames[0].payload, &[0xAA, 0xBB]);
    }

    #[test]
    fn gfdi_records_reads_file_list_response_fields() {
        let records = gfdi_records(&protobuf_frame(5043, &FILE_LIST_RESPONSE_PROTO));
        assert_eq!(
            records,
            vec![
                ("gfdi.list_response.next_page_id".to_string(), 33737.0, None),
                ("gfdi.file.size".to_string(), 4660.0, None),
                ("gfdi.file.page_id".to_string(), 33652.0, None),
            ]
        );
    }

    #[test]
    fn gfdi_records_reads_a_5044_response_frame() {
        let records = gfdi_records(&protobuf_frame(5044, &FILE_LIST_RESPONSE_PROTO));
        assert_eq!(records.len(), 3);
    }

    #[test]
    fn gfdi_records_reassembles_a_message_split_across_two_frames() {
        let proto = FILE_LIST_RESPONSE_PROTO;
        let split = 10usize;
        let mut stream = protobuf_chunk(5043, proto.len(), &proto[..split]);
        stream.extend_from_slice(&protobuf_chunk(5043, proto.len(), &proto[split..]));
        assert_eq!(
            gfdi_records(&stream),
            vec![
                ("gfdi.list_response.next_page_id".to_string(), 33737.0, None),
                ("gfdi.file.size".to_string(), 4660.0, None),
                ("gfdi.file.page_id".to_string(), 33652.0, None),
            ]
        );
    }

    #[test]
    fn gfdi_records_reassembles_a_message_split_across_three_frames() {
        let proto = FILE_LIST_RESPONSE_PROTO;
        let mut stream = protobuf_chunk(5044, proto.len(), &proto[..7]);
        stream.extend_from_slice(&protobuf_chunk(5044, proto.len(), &proto[7..13]));
        stream.extend_from_slice(&protobuf_chunk(5044, proto.len(), &proto[13..]));
        assert_eq!(gfdi_records(&stream).len(), 3);
    }

    #[test]
    fn gfdi_records_leaves_a_truncated_message_absent() {
        let proto = FILE_LIST_RESPONSE_PROTO;
        let stream = protobuf_chunk(5043, proto.len(), &proto[..10]);
        assert!(gfdi_records(&stream).is_empty());
    }

    #[test]
    fn gfdi_records_drops_a_reassembly_broken_by_a_non_protobuf_frame() {
        let proto = FILE_LIST_RESPONSE_PROTO;
        let mut stream = protobuf_chunk(5043, proto.len(), &proto[..10]);
        stream.extend_from_slice(&gfdi_frame(5000, &[0xA0, 0x13, 0x00]));
        stream.extend_from_slice(&protobuf_chunk(5043, proto.len(), &proto[10..]));
        assert!(gfdi_records(&stream).is_empty());
    }

    #[test]
    fn gfdi_records_leaves_a_file_type_name_string_absent() {
        let proto = [
            0xDA, 0x02, 0x11, 0x52, 0x0F, 0x22, 0x0D, 0x12, 0x0B, 0x18, 0x08, 0x12, 0x07, b'm',
            b'o', b'n', b'i', b't', b'o', b'r',
        ];
        let records = gfdi_records(&protobuf_frame(5043, &proto));
        assert_eq!(
            records,
            vec![("gfdi.file.type.code".to_string(), 8.0, None)]
        );
    }

    #[test]
    fn gfdi_records_skips_a_zero_status_through_the_plausibility_gate() {
        let proto = [0xDA, 0x02, 0x04, 0x12, 0x02, 0x08, 0x00];
        assert!(gfdi_records(&protobuf_frame(5043, &proto)).is_empty());
    }

    #[test]
    fn gfdi_records_returns_nothing_for_garbage() {
        assert!(gfdi_records(&[]).is_empty());
        assert!(gfdi_records(&[0xFF; 64]).is_empty());
        assert!(gfdi_records(&[0x80]).is_empty());
    }

    #[test]
    fn gfdi_records_returns_nothing_for_a_non_protobuf_frame() {
        let ack = [0x09, 0x00, 0x88, 0x13, 0xA0, 0x13, 0x00, 0x70, 0x89];
        assert!(gfdi_records(&ack).is_empty());
    }
}
