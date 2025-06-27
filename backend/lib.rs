use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static GREETING: RefCell<ic_stable_structures::Cell<String, Memory>> = RefCell::new(
        ic_stable_structures::Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            "Hello, ".to_string()
        ).unwrap()
    );

    // Хранилище для истории приветствий
    static GREETING_HISTORY: RefCell<StableBTreeMap<u64, String, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

#[ic_cdk::update]
fn set_greeting(prefix: String) {
    if prefix.len() > 50 {
        panic!("Prefix too long! Max 50 characters.");
    }
    GREETING.with_borrow_mut(|greeting| greeting.set(prefix).unwrap());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    if name.len() > 50 {
        panic!("Name too long! Max 50 characters.");
    }
    let greeting = GREETING.with_borrow(|greeting| format!("{}{name}!", greeting.get()));
    // Сохраняем приветствие в историю
    GREETING_HISTORY.with_borrow_mut(|history| {
        let id = history.len();
        history.insert(id, greeting.clone());
    });
    greeting
}

#[ic_cdk::query]
fn get_greeting_history() -> Vec<String> {
    GREETING_HISTORY.with_borrow(|history| {
        history.iter().map(|(_, v)| v.clone()).collect()
    })
}

ic_cdk::export_candid!();