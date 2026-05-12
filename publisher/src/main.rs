use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        "user_created".to_string()
    }
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher(
        "amqp://guest:guest@localhost:5672".to_owned()
    ).expect("Gagal menghubungkan ke RabbitMQ");

    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "1".to_owned(), 
        user_name: "Ahmad Anggara Bayuadji Prawirosoenoto [2406495514]-1".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "2".to_owned(), 
        user_name: "Ahmad Anggara Bayuadji Prawirosoenoto [2406495514]-2".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "3".to_owned(), 
        user_name: "Ahmad Anggara Bayuadji Prawirosoenoto [2406495514]-3".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "4".to_owned(), 
        user_name: "Ahmad Anggara Bayuadji Prawirosoenoto [2406495514]-4".to_owned() 
    });
    
    _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "5".to_owned(), 
        user_name: "Ahmad Anggara Bayuadji Prawirosoenoto [2406495514]-5".to_owned() 
    });

    println!("5 Pesan berhasil dikirim ke antrean 'user_created'!");
}