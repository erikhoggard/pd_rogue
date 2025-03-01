use pd::system::System;
use pd::system::api::Default as DefaultApi;

static mut GLOBAL_SEED: u64 = 0;

fn init_seed() -> u64 {
    let sys: System<DefaultApi> = System::default();
    System::current_time(&sys).as_millis() as u64
}

pub fn random_0_to_1() -> f64 {
    // linear Congruential Generator (LCG) parameters.
    const A: u64 = 6364136223846793005;
    const C: u64 = 1;

    unsafe {
        if GLOBAL_SEED == 0 {
            GLOBAL_SEED = init_seed();
        }

        GLOBAL_SEED = GLOBAL_SEED.wrapping_mul(A).wrapping_add(C);
        let x = GLOBAL_SEED;

        (x as f64) / (u64::MAX as f64)
    }
}