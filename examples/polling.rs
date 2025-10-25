use async_trait::async_trait;
use shadowsocks_port::log::set_tracing_subscriber;
use tracing::{Level, debug};

#[tokio::main]
async fn main() -> ! {
    set_tracing_subscriber(Level::DEBUG);

    loop {
        let poll = Poll::new(Box::new(PollTraitImplA));

        let test = "test";
        tokio::spawn(async move {
            sub_test(poll.model, test.to_string()).await;
        });

        debug!("This is a polling loop");

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn sub_test(poll_trait: Box<dyn PollTrait + Send>, test: String) {
    loop {
        debug!("Hello, {}!", test);
        let result = poll_trait.read(test.clone()).await.unwrap();
        debug!("Result: {}", result);
        tokio::time::sleep(std::time::Duration::from_secs(7)).await;
    }
}

struct Poll {
    model: Box<dyn PollTrait + Send>,
}

impl Poll {
    pub fn new(model: Box<dyn PollTrait + Send>) -> Self {
        Self { model }
    }
}

#[async_trait]
pub trait PollTrait {
    async fn read(&self, test: String) -> Result<String, Box<dyn std::error::Error>>;
}

struct PollTraitImplA;

#[async_trait]
impl PollTrait for PollTraitImplA {
    async fn read(&self, test: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("PollTraitImplA, {}!", test))
    }
}

struct PollTraitImplB;

#[async_trait]
impl PollTrait for PollTraitImplB {
    async fn read(&self, test: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("PollTraitImplB, {}!", test))
    }
}
