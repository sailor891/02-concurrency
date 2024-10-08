use anyhow::Result;
use concurrency::*;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let matrics_names = ["req.page.1", "req.page.2", "req.page.3", "req.page.4"];
    let metrics = AtomicMetrics::new(&matrics_names);
    for _i in 0..N {
        worker1(_i, metrics.clone())?;
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
fn worker1(_idx: usize, metrics: AtomicMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            metrics.add("req.page.1")?;
            metrics.add("req.page.2")?;

            thread::sleep(Duration::from_secs(1));
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
fn worker2(metrics: AtomicMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.add(format!("req.page.{}", page).as_str())?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
