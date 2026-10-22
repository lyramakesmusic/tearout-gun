use nih_plug::prelude::*;
use std::sync::atomic::Ordering;
use tearout_plug::Tearout;

fn main() {
    tearout_plug::STANDALONE.store(true, Ordering::Relaxed);
    let mut args: Vec<String> = std::env::args().collect();
    // run at the output device's own rate unless one is asked for
    if !args.iter().any(|a| a == "-r" || a.starts_with("--sample-rate")) {
        use cpal::traits::{DeviceTrait, HostTrait};
        if let Some(rate) = cpal::default_host().default_output_device().and_then(|d| d.default_output_config().ok()).map(|c| c.sample_rate().0) {
            args.extend(["--sample-rate".to_string(), rate.to_string()]);
        }
    }
    nih_export_standalone_with_args::<Tearout, _>(args);
}
