use dashmap::DashMap;
use std::{
    rc::{Rc, Weak},
    time::Duration,
};
mod resource;
use resource::{ExpensiveResource, Uuid, RESOURCE_TTL_MS};

// Tip 1:
// You can use dashmap::DashMap as hashmap to simplify working with maps;

// Tip 2:
// You can spawn async tasks with tokio::task::spawn_local;

// Feel free to create/remove any necessary internal structures

// You don't necesserally need it
struct ExpensiveResourceWrapped {

}


struct Pool {
    // Your code here
}

impl Pool {
    fn new() -> Self {
        Self {
        }
    }

    fn get_resource(self: &Rc<Self>, uuid: Uuid) -> Rc<ExpensiveResourceWrapped> {
        // Your code here
        Rc::new(ExpensiveResourceWrapped{})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resource::GLOBAL_RESOURCE_MAP;

    #[test]
    fn it_works() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap();
        rt.block_on(async {
            let mp = Rc::new(Pool::new());
            let set = tokio::task::LocalSet::new();

            // create (10..20) resources
            (10..20).for_each(|i| {
                let mp = mp.clone();
                set.spawn_local(async move {
                    println!("asking for resource {}", i);
                    let r = mp.get_resource(i);
                    tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS * 2)).await;
                    println!(
                        "sleept for {} and dropping resource {}",
                        RESOURCE_TTL_MS * 2,
                        i
                    );
                    drop(r);
                });
            });

            // create (0..10) resources and reuse (10..20) resources
            (0..20).for_each(|i| {
                let mp = mp.clone();
                set.spawn_local({
                    async move {
                        tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS / 3))
                            .await;
                        println!(
                            "sleept for {} and asking for resource {}",
                            RESOURCE_TTL_MS / 3,
                            i
                        );
                        mp.get_resource(i);
                    }
                });
            });

            // reuse (0..10) resources
            (0..10).for_each(|i| {
                let mp = mp.clone();
                set.spawn_local(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS / 2)).await;
                    println!(
                        "sleept for {} and asking for resource {}",
                        RESOURCE_TTL_MS / 2,
                        i
                    );
                    mp.get_resource(i);
                });
            });

            set.await;
            // check that each resource created only once;
            (0..20).for_each(|i| {
                let v = GLOBAL_RESOURCE_MAP.get(&i).unwrap();
                assert_eq!(v.0, 1);
            });

            tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS)).await;
            // check that all resources are deleted once;
            (0..20).for_each(|i| {
                let v = GLOBAL_RESOURCE_MAP.get(&i).unwrap();
                assert_eq!(v.1, 1);
            });
        });
    }
}
