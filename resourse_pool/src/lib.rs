use dashmap::DashMap;
use std::{
    rc::{Rc, Weak},
    time::Duration,
};
mod resource;
use resource::{ExpensiveResource, Uuid, RESOURCE_TTL_MS};

pub struct ExpensiveResourceWrapped {
    resource: Rc<ExpensiveResource>,
    pool: Rc<Pool>,
    uuid: Uuid,
}

impl ExpensiveResourceWrapped {
    fn new(resource: Rc<ExpensiveResource>, pool: Rc<Pool>, uuid: Uuid) -> Self {
        Self {
            resource,
            pool,
            uuid,
        }
    }
}

impl Drop for ExpensiveResourceWrapped {
    fn drop(&mut self) {
        let uuid = self.uuid;
        let pool = self.pool.clone();
        pool.inuse_arena.remove(&uuid);

        // we're not cleaning up values immiedietly because setting up data producer is expensive and new user may show up (think of browser refresh)
        self.pool.cleanup_resourses.insert(
            uuid,
            tokio::task::spawn_local(async move {
                tokio::time::sleep(Duration::from_millis(RESOURCE_TTL_MS)).await;
                pool.cold_arena.remove(&uuid);
            }),
        );
    }
}

struct Pool {
    cleanup_resourses: DashMap<Uuid, tokio::task::JoinHandle<()>>,
    cold_arena: DashMap<Uuid, Rc<ExpensiveResource>>, // stores expensive to construct prototypes, if value deleted from here then we need to recreate the controller
    inuse_arena: DashMap<Uuid, Weak<ExpensiveResourceWrapped>>, // stores active values, if value is deleted than recreate quickly by cloning prototype
}

impl Pool {
    fn new() -> Self {
        Self {
            cleanup_resourses: DashMap::new(),
            cold_arena: DashMap::new(),
            inuse_arena: DashMap::new(),
        }
    }

    fn get_resource(self: &Rc<Self>, uuid: Uuid) -> Rc<ExpensiveResourceWrapped> {
        match self.cold_arena.get_mut(&uuid) {
            // no resource anywhere, create it and put to cold and inuse arenas
            None => {
                let resource = Rc::new(ExpensiveResource::new(uuid));
                let new = Rc::new(ExpensiveResourceWrapped::new(
                    resource.clone(),
                    self.clone(),
                    uuid,
                ));
                self.cold_arena.insert(uuid, resource);
                self.inuse_arena.insert(uuid, Rc::downgrade(&new));
                new
            }
            // we have it in cold arena, check if it is in inuse_arena
            Some(resource) => match self.inuse_arena.get(&uuid).and_then(|v| v.upgrade()) {
                Some(v) => v,
                None => {
                    if let Some((_, d)) = self.cleanup_resourses.remove(&uuid) {
                        d.abort();
                    }
                    let wrapped = Rc::new(ExpensiveResourceWrapped::new(
                        resource.clone(),
                        self.clone(),
                        uuid,
                    ));
                    self.inuse_arena.insert(uuid, Rc::downgrade(&wrapped));
                    wrapped
                }
            },
        }
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
