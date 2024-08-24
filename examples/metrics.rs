use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::new();
    for i in 0..N {
        worker1(i, metrics.clone())?;
    }
    for _i in 0..M {
        worker2(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{}", metrics);
    }
    #[allow(unreachable_code)]
    Ok(())
}
fn worker1(_idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            metrics.add("req.page.1");
            metrics.add("req.page.2");

            thread::sleep(Duration::from_secs(1));
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
fn worker2(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.add(format!("req.page.{}", page).as_str());
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
