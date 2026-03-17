const THREAD_COUNT:usize = 15;

fn main() {
    let total_count: u64 = 1_000_000_000;


    let mut less_count_vec: Vec<u64> = vec![0; THREAD_COUNT];

    let mut handles = vec![];
    for _t in 0..(THREAD_COUNT)
    {
        handles.push(std::thread::spawn(move || {
            let mut count: u64 = 0;
            for _n in 0..=total_count
            {
                let x: f64 = rand::random();
                let y: f64 = rand::random();
                let z: f64 = rand::random();
                let w: f64 = rand::random();
                let q = x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2);
                if q <= 1.0
                {
                    count +=1;
                }
            }
            count
        }));

    }
    for (x, handle) in handles.into_iter().enumerate()
    {
        less_count_vec[x] = handle.join().unwrap();
    }

    let mut less_count: u64 = 0;
    for _i in 0..=(less_count_vec.len()-1)
    {
        less_count += less_count_vec[_i];
    }

    let p: f64 = (less_count as f64) / (((THREAD_COUNT as u64) * total_count) as f64);
    print!("{} \n", p);
}
