use dashmap::DashMap;
use lazy_static::lazy_static;

pub type Uuid = usize;

pub const RESOURCE_TTL_MS: u64 = 250;

// ugly for tests only
lazy_static! {
    pub static ref GLOBAL_RESOURCE_MAP: DashMap<Uuid, (usize, usize)> = DashMap::new();
}

pub struct ExpensiveResource {
    uuid: Uuid,
}

impl ExpensiveResource {
    pub fn new(uuid: Uuid) -> Self {
        if let Some(mut v) = GLOBAL_RESOURCE_MAP.get_mut(&uuid) {
            v.0 += 1;
        } else {
            GLOBAL_RESOURCE_MAP.insert(uuid, (1, 0));
        }

        println!("resource {} constructed", uuid);
        Self { uuid }
    }
}

impl Drop for ExpensiveResource {
    fn drop(&mut self) {
        println!("resource {} dropped", self.uuid);
        GLOBAL_RESOURCE_MAP.get_mut(&self.uuid).unwrap().1 += 1;
    }
}
