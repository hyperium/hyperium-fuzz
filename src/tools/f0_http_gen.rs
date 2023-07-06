use hyperium_fuzz_utils::f0_http_generator::GrammarGenerator;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut rng = rand_pcg::Pcg64Mcg::new(0xdeadbeef12340);
    let mut data: Vec<u8> = GrammarGenerator::generate_new(Some(512 as usize), &mut rng);

    for i in 0..1_000_000 {
        GrammarGenerator::generate_into(&mut data, Some(512 as usize), &mut rng);
        let fname = format!("./fuzz/corpus/fuzz_hyper_request/{}", i);
        let mut file = File::create(&fname)?;
        file.write_all(&data)?;
    }

    Ok(())
}
