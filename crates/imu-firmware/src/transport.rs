use heapless::{String, Vec};
use imu_core::{
    BusDescriptor, ErrorFrame, HeartbeatFrame, HelloFrame, ImuDescriptor, ImuError, ImuId,
    ImuKind, ProbeResultFrame, PROTOCOL_VERSION, RawSample, SampleFrame, TopologyFrame,
    WireFormat, WireFrame, WireHeader, MAX_MESSAGE_LEN,
};

pub struct SessionRuntime {
    pub system_id: u16,
    pub session_id: u32,
    pub format: WireFormat,
    seq: u32,
}

impl SessionRuntime {
    pub const fn new(system_id: u16, session_id: u32, format: WireFormat) -> Self {
        Self {
            system_id,
            session_id,
            format,
            seq: 0,
        }
    }

    pub fn header(&mut self, uptime_ms: u32) -> WireHeader {
        let header = WireHeader {
            protocol_version: PROTOCOL_VERSION,
            format: self.format,
            system_id: self.system_id,
            session_id: self.session_id,
            seq: self.seq,
            uptime_ms,
        };
        self.seq = self.seq.wrapping_add(1);
        header
    }

    pub fn hello(&mut self, uptime_ms: u32, system_label: &str) -> WireFrame {
        WireFrame::Hello(HelloFrame {
            header: self.header(uptime_ms),
            system_label: heapless_string::<32>(system_label),
        })
    }

    pub fn topology<const B: usize, const I: usize>(
        &mut self,
        uptime_ms: u32,
        buses: Vec<BusDescriptor, B>,
        imus: Vec<ImuDescriptor, I>,
    ) -> WireFrame {
        let mut bus_vec = Vec::new();
        for bus in buses {
            let _ = bus_vec.push(bus);
        }

        let mut imu_vec = Vec::new();
        for imu in imus {
            let _ = imu_vec.push(imu);
        }

        WireFrame::Topology(TopologyFrame {
            header: self.header(uptime_ms),
            buses: bus_vec,
            imus: imu_vec,
        })
    }

    pub fn probe_result(
        &mut self,
        uptime_ms: u32,
        imu_id: ImuId,
        driver_name: &str,
        detected_kind: ImuKind,
        success: bool,
        error: Option<ImuError>,
        profile: Option<imu_core::BusProfile>,
    ) -> WireFrame {
        WireFrame::ProbeResult(ProbeResultFrame {
            header: self.header(uptime_ms),
            imu_id,
            driver_name: heapless_string::<32>(driver_name),
            detected_kind,
            success,
            error,
            profile,
        })
    }

    pub fn sample(
        &mut self,
        uptime_ms: u32,
        imu_id: ImuId,
        imu_kind: ImuKind,
        sample_index: u32,
        timestamp_us: u64,
        sample: RawSample,
        status_bits: u16,
    ) -> WireFrame {
        WireFrame::Sample(SampleFrame {
            header: self.header(uptime_ms),
            imu_id,
            imu_kind,
            sample_index,
            timestamp_us,
            sample,
            status_bits,
        })
    }

    pub fn error(
        &mut self,
        uptime_ms: u32,
        imu_id: Option<ImuId>,
        error: ImuError,
        message: &str,
    ) -> WireFrame {
        WireFrame::Error(ErrorFrame {
            header: self.header(uptime_ms),
            imu_id,
            error,
            message: heapless_string::<MAX_MESSAGE_LEN>(message),
        })
    }

    pub fn heartbeat(&mut self, uptime_ms: u32, active_imus: u16) -> WireFrame {
        WireFrame::Heartbeat(HeartbeatFrame {
            header: self.header(uptime_ms),
            active_imus,
        })
    }
}

pub fn heapless_string<const N: usize>(value: &str) -> String<N> {
    let mut output = String::new();
    for ch in value.chars() {
        if output.push(ch).is_err() {
            break;
        }
    }
    output
}
