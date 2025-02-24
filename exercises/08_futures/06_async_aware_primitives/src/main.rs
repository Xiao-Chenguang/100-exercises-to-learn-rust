/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        let msg = receiver.recv().await.unwrap();

        println!("Pong received: {}", msg.payload);
        let (sender, new_receiver) = mpsc::channel(1);
        msg.response_channel
            .send(Message {
                payload: "pong".into(),
                response_channel: sender,
            })
            .await
            .unwrap();
        receiver = new_receiver;
    }
}

async fn ping() {
    let (sender, receiver) = mpsc::channel(1);
    let (response_sender, mut response_receiver) = mpsc::channel(1);
    eprintln!("channel created!");
    sender
        .send(Message {
            payload: "pong".into(),
            response_channel: response_sender,
        })
        .await
        .unwrap();

    eprintln!("msg sent!");
    tokio::spawn(pong(receiver));
    eprintln!("thread created!");

    let answer = response_receiver.recv().await.unwrap().payload;
    eprintln!("answer get!");
    assert_eq!(answer, "pong");
}

// run ping in main
#[tokio::main]
async fn main() {
    ping().await;
}
