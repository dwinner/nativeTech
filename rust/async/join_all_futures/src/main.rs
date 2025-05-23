extern crate trpl;

use std::pin::{Pin, pin};
use std::time::Duration;

fn main()
{
   trpl::run(async {
      let (tx, mut rx) = trpl::channel();
      let tx1 = tx.clone();

      let tx1_fut = pin!(async move {
         let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
         ];

         for val in vals
         {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_secs(1)).await;
         }
      });

      let rx_fut = pin!(async {
         while let Some(value) = rx.recv().await
         {
            println!("received '{value}'");
         }
      });

      let tx_fut = pin!(async move {
         let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
         ];

         for val in vals
         {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_secs(1)).await;
         }
      });

      let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

      trpl::join_all(futures).await;
   });
}
