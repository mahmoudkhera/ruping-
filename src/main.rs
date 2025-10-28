use ruping::icmp::{KIND, RAWICMP};

fn main() {
    let mut raw_icmp = RAWICMP::new(KIND::ECHO);

    let mut slice = vec![1];

    let buff = raw_icmp.evaluate_icmp(&mut slice);
    println!("{:?}", buff);
}
