#[derive(Debug)]
pub struct RangeMap {
    pub entries: Vec<RangeMapEntry>,
}
#[derive(Debug)]
pub struct RangeMapEntry {
    pub source_start: u32,
    pub destination_start: u32,
    pub range_length: u32,
}

impl RangeMap {
    pub fn find_destination(&self, seed: u32) -> u32 {
        self.entries
            .iter()
            .find_map(|entry| {
                if entry.source_start <= seed {
                    let distance = seed - entry.source_start;
                    if distance < entry.range_length {
                        return Some(entry.destination_start + distance);
                    }
                }
                None
            })
            .unwrap_or(seed)
    }
}
