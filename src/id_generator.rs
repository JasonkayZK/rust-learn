use std::sync::OnceLock;

use rand::Rng;
use snowflake::SnowflakeIdGenerator;
use tokio::sync::Mutex;

static GLOBAL_ID_GENERATOR: OnceLock<Mutex<GlobalId>> = OnceLock::new();

pub struct GlobalId {
    generator: SnowflakeIdGenerator,
}

impl GlobalId {
    pub async fn next_id() -> u64 {
        Self::global().lock().await.generator.generate() as u64
    }

    fn global() -> &'static Mutex<Self> {
        GLOBAL_ID_GENERATOR.get_or_init(|| {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen();
            Mutex::new(Self {
                generator: SnowflakeIdGenerator::new(1, random_number),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::id_generator::GlobalId;

    #[tokio::test]
    async fn test_generate_id() {
        let id = GlobalId::next_id().await;
        println!("{}", id)
    }
}
