// Note names typed and shown: parse_note round-trips, and frequencies show their nearest note.
use tearout_plug::params::{hz_note, note_name, parse_note};
fn main() {
    for t in ["F#4", "bb2", "c-1", "A4", "g#0", "B", "e7"] { println!("{t:5} -> {:?} -> {}", parse_note(t), parse_note(t).map(note_name).unwrap_or_default()); }
    for f in [350.0, 800.0, 440.0, 55.0, 11092.0] { println!("{f} Hz -> {}", hz_note(f)); }
}
