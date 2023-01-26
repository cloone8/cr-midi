pub enum Chunk {
    Header(Header),
    Track(Track)
}

pub struct Header {
    pub format: SmfFormat,
    pub track_count: u16,
    pub division: Division
}

pub enum Division {
    TicksPerQuarter(u16),
    FramesPerSecond(u8, u8)
}

pub enum SmfFormat {
    SingleTrack,
    MultiTrack,
    MultiSong
}

pub struct Track {
    pub events: Vec<TrackEvent>
}

pub struct TrackEvent {
    pub time: u64,
    pub event: Event
}

pub enum Event {

}
