enum WineRed {
    Merlot,
    CabernetSauvignon,
    PinotNoir,
}
struct Wine {
    name: WineRed,
    year: u16,
    region: WineRed,
}
fn supportRegion(w: WineRed) -> bool {
    match w {
        WineRed::Merlot => true,
        WineRed::CabernetSauvignon => true,
        WineRed::PinotNoir => true,
    }
}
