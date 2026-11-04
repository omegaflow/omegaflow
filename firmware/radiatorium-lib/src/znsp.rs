pub const SLIP_END: u8 = 0xC0;
pub const SLIP_ESC: u8 = 0xDB;
pub const SLIP_ESC_END: u8 = 0xDC;
pub const SLIP_ESC_ESC: u8 = 0xDD;

pub const SLIP_BUF: usize = 512;
pub const HEADER_LEN: usize = 7;
pub const CRC_LEN: usize = 2;
pub const ZNSP_VERSION: u8 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slip {
    Pending,
    Packet,
    Resync,
}

pub struct SlipDecoder {
    buf: [u8; SLIP_BUF],
    len: usize,
    escaping: bool,
    resync: bool,
    complete: bool,
}

impl SlipDecoder {
    pub const fn new() -> Self {
        Self {
            buf: [0; SLIP_BUF],
            len: 0,
            escaping: false,
            resync: false,
            complete: false,
        }
    }

    pub fn packet(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn push(&mut self, byte: u8) -> Slip {
        if self.complete {
            self.len = 0;
            self.complete = false;
        }
        if byte == SLIP_END {
            if self.resync {
                self.resync = false;
                self.escaping = false;
                self.len = 0;
                return Slip::Resync;
            }
            self.escaping = false;
            if self.len == 0 {
                return Slip::Pending;
            }
            self.complete = true;
            return Slip::Packet;
        }
        if self.resync {
            return Slip::Resync;
        }
        if byte == SLIP_ESC {
            if self.escaping {
                self.escaping = false;
                return self.store(byte);
            }
            self.escaping = true;
            return Slip::Pending;
        }
        if self.escaping {
            self.escaping = false;
            let decoded = match byte {
                SLIP_ESC_END => SLIP_END,
                SLIP_ESC_ESC => SLIP_ESC,
                other => other,
            };
            return self.store(decoded);
        }
        self.store(byte)
    }

    fn store(&mut self, byte: u8) -> Slip {
        if self.len >= SLIP_BUF {
            self.resync = true;
            self.escaping = false;
            self.len = 0;
            return Slip::Resync;
        }
        self.buf[self.len] = byte;
        self.len += 1;
        Slip::Pending
    }
}

pub fn crc16_le(data: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0x8408;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Request,
    Response,
    Notify,
}

impl FrameType {
    pub fn from_nibble(nibble: u8) -> Option<FrameType> {
        match nibble {
            0 => Some(FrameType::Request),
            1 => Some(FrameType::Response),
            2 => Some(FrameType::Notify),
            _ => None,
        }
    }

    pub const fn nibble(self) -> u8 {
        match self {
            FrameType::Request => 0,
            FrameType::Response => 1,
            FrameType::Notify => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame<'a> {
    pub version: u8,
    pub kind: FrameType,
    pub id: u16,
    pub sn: u8,
    pub payload: &'a [u8],
}

impl<'a> Frame<'a> {
    pub fn frame_len(packet: &[u8]) -> Option<usize> {
        if packet.len() < HEADER_LEN {
            return None;
        }
        let len = u16::from_le_bytes([packet[5], packet[6]]) as usize;
        HEADER_LEN.checked_add(len)?.checked_add(CRC_LEN)
    }

    pub fn parse(packet: &'a [u8]) -> Option<Frame<'a>> {
        let total = Frame::frame_len(packet)?;
        if packet.len() < total {
            return None;
        }
        let crc = u16::from_le_bytes([packet[total - 2], packet[total - 1]]);
        if crc != crc16_le(&packet[..total - CRC_LEN]) {
            return None;
        }
        let flags = u16::from_le_bytes([packet[0], packet[1]]);
        let kind = FrameType::from_nibble(((flags >> 4) & 0x000F) as u8)?;
        let version = (flags & 0x000F) as u8;
        let id = u16::from_le_bytes([packet[2], packet[3]]);
        let sn = packet[4];
        let payload = &packet[HEADER_LEN..total - CRC_LEN];
        Some(Frame {
            version,
            kind,
            id,
            sn,
            payload,
        })
    }

    pub fn encode(
        kind: FrameType,
        id: u16,
        sn: u8,
        payload: &[u8],
        out: &mut [u8],
    ) -> Option<usize> {
        let len = payload.len();
        if len > u16::MAX as usize {
            return None;
        }
        let total = HEADER_LEN.checked_add(len)?.checked_add(CRC_LEN)?;
        if out.len() < total {
            return None;
        }
        let flags: u16 = ZNSP_VERSION as u16 | ((kind.nibble() as u16) << 4);
        let flag_bytes = flags.to_le_bytes();
        out[0] = flag_bytes[0];
        out[1] = flag_bytes[1];
        let id_bytes = id.to_le_bytes();
        out[2] = id_bytes[0];
        out[3] = id_bytes[1];
        out[4] = sn;
        let len_bytes = (len as u16).to_le_bytes();
        out[5] = len_bytes[0];
        out[6] = len_bytes[1];
        out[HEADER_LEN..HEADER_LEN + len].copy_from_slice(payload);
        let crc = crc16_le(&out[..HEADER_LEN + len]);
        let crc_bytes = crc.to_le_bytes();
        out[HEADER_LEN + len] = crc_bytes[0];
        out[HEADER_LEN + len + 1] = crc_bytes[1];
        Some(total)
    }
}

pub fn slip_encode(packet: &[u8], out: &mut [u8]) -> Option<usize> {
    if out.is_empty() {
        return None;
    }
    out[0] = SLIP_END;
    let mut n: usize = 1;
    for &byte in packet {
        if byte == SLIP_END {
            if n + 2 > out.len() {
                return None;
            }
            out[n] = SLIP_ESC;
            out[n + 1] = SLIP_ESC_END;
            n += 2;
        } else if byte == SLIP_ESC {
            if n + 2 > out.len() {
                return None;
            }
            out[n] = SLIP_ESC;
            out[n + 1] = SLIP_ESC_ESC;
            n += 2;
        } else {
            if n + 1 > out.len() {
                return None;
            }
            out[n] = byte;
            n += 1;
        }
    }
    if n + 1 > out.len() {
        return None;
    }
    out[n] = SLIP_END;
    n += 1;
    Some(n)
}

pub mod cmd {
    pub const NETWORK_INIT: u16 = 0x0000;
    pub const NETWORK_START: u16 = 0x0001;
    pub const NETWORK_STATE: u16 = 0x0002;
    pub const NETWORK_STACK_STATUS_HANDLER: u16 = 0x0003;
    pub const NETWORK_FORMNETWORK: u16 = 0x0004;
    pub const NETWORK_PERMIT_JOINING: u16 = 0x0005;
    pub const NETWORK_JOINNETWORK: u16 = 0x0006;
    pub const NETWORK_LEAVENETWORK: u16 = 0x0007;
    pub const NETWORK_START_SCAN: u16 = 0x0008;
    pub const NETWORK_SCAN_COMPLETE_HANDLER: u16 = 0x0009;
    pub const NETWORK_STOP_SCAN: u16 = 0x000A;
    pub const NETWORK_PAN_ID_GET: u16 = 0x000B;
    pub const NETWORK_PAN_ID_SET: u16 = 0x000C;
    pub const NETWORK_EXTENDED_PAN_ID_GET: u16 = 0x000D;
    pub const NETWORK_EXTENDED_PAN_ID_SET: u16 = 0x000E;
    pub const NETWORK_CHANNEL_GET: u16 = 0x0013;
    pub const NETWORK_CHANNEL_SET: u16 = 0x0014;
    pub const NETWORK_ROLE_GET: u16 = 0x001B;
    pub const NETWORK_SHORT_ADDRESS_GET: u16 = 0x001D;
    pub const NETWORK_LONG_ADDRESS_GET: u16 = 0x001F;
    pub const ERROR: u16 = 0xFFFF;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success,
    ErrFatal,
    BadArgument,
    ErrNoMem,
}

impl Status {
    pub fn from_byte(byte: u8) -> Option<Status> {
        match byte {
            0x00 => Some(Status::Success),
            0x01 => Some(Status::ErrFatal),
            0x02 => Some(Status::BadArgument),
            0x03 => Some(Status::ErrNoMem),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    InitSent,
    FormNetworkSent,
    Started,
    Steering,
    Joined,
}

pub const ESP_ZB_CFG_SIZE: usize = 16;
pub const ESP_ZB_CFG_ROLE_OFFSET: usize = 0;
pub const ESP_ZB_CFG_INSTALL_CODE_OFFSET: usize = 4;
pub const ESP_ZB_CFG_NWK_OFFSET: usize = 8;
pub const ESP_ZB_CFG_ZCZR_MAX_CHILDREN_OFFSET: usize = 8;
pub const ESP_ZB_CFG_ZED_ED_TIMEOUT_OFFSET: usize = 8;
pub const ESP_ZB_CFG_ZED_KEEP_ALIVE_OFFSET: usize = 12;

pub const ESP_ZB_DEVICE_TYPE_COORDINATOR: u32 = 0x0;
pub const ESP_ZB_DEVICE_TYPE_ROUTER: u32 = 0x1;
pub const ESP_ZB_DEVICE_TYPE_ED: u32 = 0x2;

pub const fn form_network_payload_zczr(
    role: u32,
    install_code_policy: bool,
    max_children: u8,
) -> [u8; ESP_ZB_CFG_SIZE] {
    let mut out = [0u8; ESP_ZB_CFG_SIZE];
    let role_bytes = role.to_le_bytes();
    out[ESP_ZB_CFG_ROLE_OFFSET] = role_bytes[0];
    out[ESP_ZB_CFG_ROLE_OFFSET + 1] = role_bytes[1];
    out[ESP_ZB_CFG_ROLE_OFFSET + 2] = role_bytes[2];
    out[ESP_ZB_CFG_ROLE_OFFSET + 3] = role_bytes[3];
    out[ESP_ZB_CFG_INSTALL_CODE_OFFSET] = install_code_policy as u8;
    out[ESP_ZB_CFG_ZCZR_MAX_CHILDREN_OFFSET] = max_children;
    out
}

pub const fn form_network_payload_zed(
    role: u32,
    install_code_policy: bool,
    ed_timeout: u8,
    keep_alive: u32,
) -> [u8; ESP_ZB_CFG_SIZE] {
    let mut out = [0u8; ESP_ZB_CFG_SIZE];
    let role_bytes = role.to_le_bytes();
    out[ESP_ZB_CFG_ROLE_OFFSET] = role_bytes[0];
    out[ESP_ZB_CFG_ROLE_OFFSET + 1] = role_bytes[1];
    out[ESP_ZB_CFG_ROLE_OFFSET + 2] = role_bytes[2];
    out[ESP_ZB_CFG_ROLE_OFFSET + 3] = role_bytes[3];
    out[ESP_ZB_CFG_INSTALL_CODE_OFFSET] = install_code_policy as u8;
    out[ESP_ZB_CFG_ZED_ED_TIMEOUT_OFFSET] = ed_timeout;
    let keep_alive_bytes = keep_alive.to_le_bytes();
    out[ESP_ZB_CFG_ZED_KEEP_ALIVE_OFFSET] = keep_alive_bytes[0];
    out[ESP_ZB_CFG_ZED_KEEP_ALIVE_OFFSET + 1] = keep_alive_bytes[1];
    out[ESP_ZB_CFG_ZED_KEEP_ALIVE_OFFSET + 2] = keep_alive_bytes[2];
    out[ESP_ZB_CFG_ZED_KEEP_ALIVE_OFFSET + 3] = keep_alive_bytes[3];
    out
}

pub const ESP_ZB_ZC_FORM_NETWORK_PAYLOAD: [u8; ESP_ZB_CFG_SIZE] =
    form_network_payload_zczr(ESP_ZB_DEVICE_TYPE_COORDINATOR, false, 10);

pub struct NetworkMachine {
    state: State,
}

impl NetworkMachine {
    pub const fn new() -> Self {
        Self { state: State::Idle }
    }

    pub const fn state(&self) -> State {
        self.state
    }

    pub const fn form_network_payload_pending(&self) -> Option<&'static [u8]> {
        Some(&ESP_ZB_ZC_FORM_NETWORK_PAYLOAD)
    }

    pub fn init_request(&mut self) -> Option<&'static [u8]> {
        match self.state {
            State::Idle => {
                self.state = State::InitSent;
                Some(&[])
            }
            _ => None,
        }
    }

    pub fn start_request(&mut self) -> Option<[u8; 1]> {
        match self.state {
            State::FormNetworkSent => {
                self.state = State::Started;
                Some([1])
            }
            _ => None,
        }
    }

    pub fn on_response(&mut self, frame: &Frame) -> bool {
        let status = match frame.payload.first() {
            Some(&byte) => Status::from_byte(byte),
            None => None,
        };
        match (self.state, frame.kind, frame.id, status) {
            (State::InitSent, FrameType::Response, cmd::NETWORK_INIT, Some(Status::Success)) => {
                self.state = State::FormNetworkSent;
                true
            }
            _ => false,
        }
    }

    pub fn on_notify(&mut self, frame: &Frame) -> bool {
        match (self.state, frame.kind, frame.id) {
            (State::Started, FrameType::Notify, cmd::NETWORK_FORMNETWORK) => {
                self.state = State::Steering;
                true
            }
            (State::Started, FrameType::Notify, cmd::NETWORK_PERMIT_JOINING) => {
                self.state = State::Steering;
                true
            }
            (State::Steering, FrameType::Notify, cmd::NETWORK_PERMIT_JOINING) => {
                self.state = State::Steering;
                true
            }
            (State::Steering, FrameType::Notify, cmd::NETWORK_JOINNETWORK) => {
                self.state = State::Joined;
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST_CAPTURE: [u8; 10] = [0x10, 0x00, 0x00, 0x00, 0xd0, 0x01, 0x00, 0x00, 0x23, 0x06];

    const NCP_CAPTURE: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff];

    #[test]
    fn crc_matches_host_readme_capture() {
        assert_eq!(
            crc16_le(&HOST_CAPTURE[..8]),
            u16::from_le_bytes([HOST_CAPTURE[8], HOST_CAPTURE[9]])
        );
    }

    #[test]
    fn crc_matches_ncp_readme_capture() {
        assert_eq!(
            crc16_le(&NCP_CAPTURE[..7]),
            u16::from_le_bytes([NCP_CAPTURE[7], NCP_CAPTURE[8]])
        );
    }

    #[test]
    fn frame_parses_host_readme_capture() {
        let frame = Frame::parse(&HOST_CAPTURE).expect("frame");
        assert_eq!(frame.kind, FrameType::Response);
        assert_eq!(frame.version, 0);
        assert_eq!(frame.id, cmd::NETWORK_INIT);
        assert_eq!(frame.sn, 0xd0);
        assert_eq!(frame.payload, &HOST_CAPTURE[7..8]);
    }

    #[test]
    fn bad_crc_is_rejected() {
        let mut packet = HOST_CAPTURE;
        packet[8] ^= 0xff;
        assert_eq!(Frame::parse(&packet), None);
    }

    #[test]
    fn truncated_length_is_rejected() {
        assert_eq!(Frame::parse(&HOST_CAPTURE[..HOST_CAPTURE.len() - 1]), None);
    }

    #[test]
    fn encode_parse_round_trip() {
        let payload = [0xde, 0xad, 0xc0, 0xdb, 0xbe, 0xef];
        let mut out = [0u8; 64];
        let n = Frame::encode(
            FrameType::Request,
            cmd::NETWORK_FORMNETWORK,
            7,
            &payload,
            &mut out,
        )
        .expect("encode");
        let frame = Frame::parse(&out[..n]).expect("frame");
        assert_eq!(frame.kind, FrameType::Request);
        assert_eq!(frame.version, ZNSP_VERSION);
        assert_eq!(frame.id, cmd::NETWORK_FORMNETWORK);
        assert_eq!(frame.sn, 7);
        assert_eq!(frame.payload, &payload[..]);
    }

    #[test]
    fn slip_round_trip_preserves_escapes() {
        let packet = [SLIP_END, 0x01, SLIP_ESC, 0x02, SLIP_END, SLIP_ESC, 0x03];
        let mut encoded = [0u8; 64];
        let n = slip_encode(&packet, &mut encoded).expect("encode");
        let mut decoder = SlipDecoder::new();
        let mut decoded = None;
        for &byte in &encoded[..n] {
            if let Slip::Packet = decoder.push(byte) {
                decoded = Some(decoder.packet().to_vec());
            }
        }
        assert_eq!(decoded, Some(packet.to_vec()));
    }

    #[test]
    fn unescaped_end_separates_packets() {
        let mut decoder = SlipDecoder::new();
        let mut packets = 0;
        for &byte in &[SLIP_END, 0x01, 0x02, SLIP_END, SLIP_END, 0x03, SLIP_END] {
            if let Slip::Packet = decoder.push(byte) {
                packets += 1;
            }
        }
        assert_eq!(packets, 2);
    }

    #[test]
    fn overflow_resyncs_until_end() {
        let mut decoder = SlipDecoder::new();
        let mut resynced = false;
        for _ in 0..SLIP_BUF + 4 {
            if let Slip::Resync = decoder.push(0x11) {
                resynced = true;
            }
        }
        assert!(resynced);
        assert_eq!(decoder.push(SLIP_END), Slip::Resync);
        let mut encoded = [0u8; 16];
        let n = slip_encode(&[0x42], &mut encoded).expect("encode");
        let mut decoded = None;
        for &byte in &encoded[..n] {
            if let Slip::Packet = decoder.push(byte) {
                decoded = Some(decoder.packet().to_vec());
            }
        }
        assert_eq!(decoded, Some(vec![0x42]));
    }

    #[test]
    fn status_bytes_map() {
        assert_eq!(Status::from_byte(0x00), Some(Status::Success));
        assert_eq!(Status::from_byte(0x01), Some(Status::ErrFatal));
        assert_eq!(Status::from_byte(0x02), Some(Status::BadArgument));
        assert_eq!(Status::from_byte(0x03), Some(Status::ErrNoMem));
        assert_eq!(Status::from_byte(0x04), None);
    }

    #[test]
    fn frame_type_nibbles() {
        assert_eq!(FrameType::from_nibble(0), Some(FrameType::Request));
        assert_eq!(FrameType::from_nibble(1), Some(FrameType::Response));
        assert_eq!(FrameType::from_nibble(2), Some(FrameType::Notify));
        assert_eq!(FrameType::from_nibble(3), None);
        assert_eq!(FrameType::Notify.nibble(), 2);
    }

    #[test]
    fn network_machine_formation_sequence() {
        let mut host = NetworkMachine::new();
        assert_eq!(host.state(), State::Idle);
        assert_eq!(host.start_request(), None);
        assert_eq!(
            host.form_network_payload_pending(),
            Some(&ESP_ZB_ZC_FORM_NETWORK_PAYLOAD[..])
        );
        let init_payload = host.init_request().expect("init payload");
        assert_eq!(init_payload.len(), 0);
        assert_eq!(host.state(), State::InitSent);

        let init_ok = Frame {
            version: 0,
            kind: FrameType::Response,
            id: cmd::NETWORK_INIT,
            sn: 0,
            payload: &[0x00],
        };
        assert!(host.on_response(&init_ok));
        assert_eq!(host.state(), State::FormNetworkSent);

        assert_eq!(host.start_request(), Some([1]));
        assert_eq!(host.state(), State::Started);

        let permit = Frame {
            version: 0,
            kind: FrameType::Notify,
            id: cmd::NETWORK_PERMIT_JOINING,
            sn: 0,
            payload: &[0xb4],
        };
        assert!(host.on_notify(&permit));
        assert_eq!(host.state(), State::Steering);

        let join = Frame {
            version: 0,
            kind: FrameType::Notify,
            id: cmd::NETWORK_JOINNETWORK,
            sn: 0,
            payload: &[0x50, 0x03, 0x00],
        };
        assert!(host.on_notify(&join));
        assert_eq!(host.state(), State::Joined);
    }

    #[test]
    fn form_network_payload_zczr_is_byte_exact() {
        let payload = ESP_ZB_ZC_FORM_NETWORK_PAYLOAD;
        assert_eq!(payload.len(), ESP_ZB_CFG_SIZE);
        assert_eq!(&payload[0..4], &[0x00, 0x00, 0x00, 0x00]);
        assert_eq!(payload[4], 0x00);
        assert_eq!(&payload[5..8], &[0x00, 0x00, 0x00]);
        assert_eq!(payload[8], 10);
        assert_eq!(&payload[9..16], &[0x00; 7]);

        let router = form_network_payload_zczr(ESP_ZB_DEVICE_TYPE_ROUTER, true, 0xfe);
        assert_eq!(&router[0..4], &[0x01, 0x00, 0x00, 0x00]);
        assert_eq!(router[4], 0x01);
        assert_eq!(router[8], 0xfe);
    }

    #[test]
    fn form_network_payload_zed_is_byte_exact() {
        let payload = form_network_payload_zed(ESP_ZB_DEVICE_TYPE_ED, true, 0x03, 0x01020304);
        assert_eq!(payload.len(), ESP_ZB_CFG_SIZE);
        assert_eq!(&payload[0..4], &[0x02, 0x00, 0x00, 0x00]);
        assert_eq!(payload[4], 0x01);
        assert_eq!(payload[8], 0x03);
        assert_eq!(&payload[9..12], &[0x00, 0x00, 0x00]);
        assert_eq!(&payload[12..16], &[0x04, 0x03, 0x02, 0x01]);
    }
}
