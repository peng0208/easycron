use easycron::*;

#[tokio::main]
async fn main() {
    let crond = CronBuilder::default().build().unwrap();
    println!("{:?}", crond.list());
}
