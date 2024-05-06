// You need to write a cache pool;
// Imagine you have some resource that is very expensive to contruct and move (e.g.: network connection that it takes milliseconds to establish
// or high poligonal model couple of megabytes size).
// Each resource is associated with unique id. You need to implement a caching pool.
// Pool::get_resource(uuid) logic:
// if resource with particular uuid is used by someone then return the same resource without constructing it;
// if no one uses that resource anymore then delete it;
// construct (ExpensiveResource::new()) only if you can't reuse existing one;

// Tip:
// You can use dashmap::DashMap as hashmap to simplify working with maps;

use dashmap::DashMap;
use std::rc::Rc;
mod resource;
use resource::{ExpensiveResource, Uuid};

// Feel free to create/remove any necessary internal structures
pub struct ExpensiveResourceWrapped {}

impl ExpensiveResourceWrapped {}

struct Pool {
    // Your code here
}

impl Pool {
    fn new() -> Self {
        Self {}
    }

    fn get_resource(self: Rc<Self>, uuid: Uuid) -> Rc<ExpensiveResourceWrapped> {
        // Your code here
        Rc::new(ExpensiveResourceWrapped {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resource::GLOBAL_RESOURCE_MAP;

    #[test]
    fn it_works() {
        let p = Rc::new(Pool::new());
        {
            let _r1 = p.clone().get_resource(100);
            let _r2 = p.clone().get_resource(100);
        }
        {
            let _r1 = p.clone().get_resource(100);
            let _r2 = p.clone().get_resource(200);
            let _r3 = p.clone().get_resource(200);
        }
        let v = GLOBAL_RESOURCE_MAP.get(&100).unwrap();
        assert_eq!(v.0, 2);
        assert_eq!(v.1, 2);
        let v = GLOBAL_RESOURCE_MAP.get(&200).unwrap();
        assert_eq!(v.0, 1);
        assert_eq!(v.1, 1);
    }
}

// Extra hometask: add postpone mode. What it means is that resource is not deleted immediatelly after everyone stopped using it
// it stays in the cache for resource::RESOURCE_TTL_MS;
// During this period consumers can get it again without construction (if this happens deletion is canceled).
// Tip for hometask:
// You can spawn async tasks with tokio::task::spawn_local;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use resource::GLOBAL_RESOURCE_MAP;

//     #[test]
//     fn postponed_mode_test() {
//         let rt = tokio::runtime::Builder::new_current_thread()
//             .enable_time()
//             .build()
//             .unwrap();
//         rt.block_on(async {
//             let mp = Rc::new(Pool::new());
//             let set = tokio::task::LocalSet::new();

//             // create (10..20) resources
//             (10..20).for_each(|i| {
//                 let mp = mp.clone();
//                 set.spawn_local(async move {
//                     println!("asking for resource {}", i);
//                     let r = mp.get_resource(i);
//                     tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS * 2)).await;
//                     println!(
//                         "sleept for {} and dropping resource {}",
//                         RESOURCE_TTL_MS * 2,
//                         i
//                     );
//                     drop(r);
//                 });
//             });

//             // create (0..10) resources and reuse (10..20) resources
//             (0..20).for_each(|i| {
//                 let mp = mp.clone();
//                 set.spawn_local({
//                     async move {
//                         tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS / 3))
//                             .await;
//                         println!(
//                             "sleept for {} and asking for resource {}",
//                             RESOURCE_TTL_MS / 3,
//                             i
//                         );
//                         mp.get_resource(i);
//                     }
//                 });
//             });

//             // reuse (0..10) resources
//             (0..10).for_each(|i| {
//                 let mp = mp.clone();
//                 set.spawn_local(async move {
//                     tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS / 2)).await;
//                     println!(
//                         "sleept for {} and asking for resource {}",
//                         RESOURCE_TTL_MS / 2,
//                         i
//                     );
//                     mp.get_resource(i);
//                 });
//             });

//             set.await;
//             // check that each resource created only once;
//             (0..20).for_each(|i| {
//                 let v = GLOBAL_RESOURCE_MAP.get(&i).unwrap();
//                 assert_eq!(v.0, 1);
//             });

//             tokio::time::sleep(std::time::Duration::from_millis(RESOURCE_TTL_MS)).await;
//             // check that all resources are deleted once;
//             (0..20).for_each(|i| {
//                 let v = GLOBAL_RESOURCE_MAP.get(&i).unwrap();
//                 assert_eq!(v.1, 1);
//             });
//         });
//     }
// }
