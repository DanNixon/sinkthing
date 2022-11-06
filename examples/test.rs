use sinkthing::{Client, Entity};

#[tokio::main]
async fn main() {
    let c = Client::new("");
    let res = c.devices().await;

    let d = &mut res.unwrap()[2];
    println!("device {:#?}", d);

    d.pause().await.unwrap();
    println!("did pause {:#?}", d);

    d.resume().await.unwrap();
    println!("did resume {:#?}", d);

    d.rename("boop").await.unwrap();
    println!("did rename {:#?}", d);
}
