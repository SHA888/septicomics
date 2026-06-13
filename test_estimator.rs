use std::path::Path;

fn main() {
    let crate_root = Path::new("crates/fed-protocol");
    println!("Testing serde roundtrip for EstimatorVariant...");
    
    // The serde test in analysis_plan.rs line 247-265 does NOT verify
    // that plan.estimator roundtrips. Let's see if the test would fail
    // if estimator was lost.
}
