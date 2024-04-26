use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions};

fn main() -> Result<(), amiquip::Error> {
    // Setup Rabbit-MQ connection
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    exchange.publish(Publish::new(b"Hello world!", "hello"))?;

    // Setup consumer
    let consumer = queue.consume(ConsumerOptions::default())?;

    println!("Press Ctrl-C to exit.");

    for (i, message) 
    in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("Received message number {} with body {}", i, body);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()?;
    Ok(())
}