// Вам нужно написать кэширующий пул
// Представьте у вас есть ресурс, который очень дорого конструировать и перемещать (например, сетевое соединение, которое требует миллисекунд на установку
// или высокополигональная модель размером несколько мегабайт)
// Каждый ресур имеет уникальный id. Вам нужно написать реализацию пула таких ресуров.
// Логика Pool::get_resource(uuid):
// Если ресурс по заданному uuid уже существует, то вернуть указатель на него не конструируя его;
// Если ресурс не конструировался, то сконструктировать его (ExpensiveResource::new())

// Пожалуйста используйте DashMap для хранения ресурсов

use dashmap::DashMap;
use std::rc::Rc;
mod resource;
use resource::{ExpensiveResource, Uuid};

struct Pool {
    // Your code here
}

impl Pool {
    fn new() -> Self {
        Self {}
    }

    fn get_resource(&self, uuid: Uuid) -> Rc<ExpensiveResource> {
        // Your code here
        Rc::new(ExpensiveResource::new(uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resource::GLOBAL_RESOURCE_MAP;

    #[test]
    fn it_works() {
        let p = Pool::new();
        let _r1 = p.get_resource(100);
        let _r2 = p.get_resource(200);
        let _r1 = p.get_resource(100);
        let _r2 = p.get_resource(200);
        let _r3 = p.get_resource(200);
        let v = GLOBAL_RESOURCE_MAP.get(&100).unwrap();
        assert_eq!(v.0, 1);
        let v = GLOBAL_RESOURCE_MAP.get(&200).unwrap();
        assert_eq!(v.0, 1);
    }
}

// Продолжение задания: теперь вам нужно удалять ресурс из пула, если его больше никто не использует

// use dashmap::DashMap;
// use std::rc::Rc;
// mod resource;
// use resource::{ExpensiveResource, Uuid};

// // Feel free to create/remove any necessary internal structures
// pub struct ExpensiveResourceWrapped {}

// impl ExpensiveResourceWrapped {}

// struct Pool {
//     // Your code here
// }

// impl Pool {
//     fn new() -> Self {
//         Self {}
//     }

//     fn get_resource(self: &Rc<Self>, uuid: Uuid) -> Rc<ExpensiveResourceWrapped> {
//         // Your code here
//         Rc::new(ExpensiveResourceWrapped {})
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use resource::GLOBAL_RESOURCE_MAP;

//     #[test]
//     fn it_works() {
//         let p = Rc::new(Pool::new());
//         {
//             let _r1 = p.clone().get_resource(100);
//             let _r2 = p.clone().get_resource(100);
//         }
//         {
//             let _r1 = p.clone().get_resource(100);
//             let _r2 = p.clone().get_resource(200);
//             let _r3 = p.clone().get_resource(200);
//         }
//         let v = GLOBAL_RESOURCE_MAP.get(&100).unwrap();
//         assert_eq!(v.0, 2);
//         assert_eq!(v.1, 2);
//         let v = GLOBAL_RESOURCE_MAP.get(&200).unwrap();
//         assert_eq!(v.0, 1);
//         assert_eq!(v.1, 1);
//     }
// }

// Дополнительная домашняя работа: добавьте отложенный режим. Ресурс удаляется не сразу после того, как его перестали использовать, а
// остается в кэше на какой-то таймаут (resource::RESOURCE_TTL_MS);
// Если за этот период кто-то запросит ресурс, то получит его без конструирования (удаление отменяется).
// Подсказка:
// Вы можете использовать асихнронные таски с помощью tokio::task::spawn_local;

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
