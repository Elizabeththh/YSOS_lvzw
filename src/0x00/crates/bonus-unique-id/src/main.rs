use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;


mod unsafe_version {
    use std::cell::RefCell;

    thread_local! {
        static COLLECTED_IDS: RefCell<Vec<u16>> = RefCell::new(Vec::new());
    }

    static mut COUNTER: u16 = 0;


    pub unsafe fn next_id_unsafe() -> u16 {
        let id = COUNTER;
        std::hint::black_box(());
        COUNTER += 1;
        id
    }

    pub fn collect_id_unsafe(id: u16) {
        COLLECTED_IDS.with(|ids| {
            ids.borrow_mut().push(id);
        });
    }

    pub fn get_collected_ids() -> Vec<u16> {
        COLLECTED_IDS.with(|ids| ids.borrow().clone())
    }
}

mod safe_version {
    use std::sync::atomic::{AtomicU16, Ordering};

    static COUNTER: AtomicU16 = AtomicU16::new(0);

    pub fn next_id_safe() -> u16 {
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}


fn test_unsafe_version() {
    println!("=== 测试 static mut 版本（不安全） ===\n");

    let num_threads = 4;
    let ids_per_thread = 1000;
    let all_ids = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let all_ids = Arc::clone(&all_ids);
            thread::spawn(move || {
                let mut local_ids = Vec::new();
                for _ in 0..ids_per_thread {

                    let id = unsafe { unsafe_version::next_id_unsafe() };
                    local_ids.push(id);
                }
                let mut ids = all_ids.lock().unwrap();
                ids.extend(local_ids);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let all_ids = all_ids.lock().unwrap();
    let total_ids = all_ids.len();
    let unique_ids = all_ids.iter().collect::<std::collections::HashSet<_>>();

    println!("总 ID 数量: {}", total_ids);
    println!("唯一 ID 数量: {}", unique_ids.len());
    println!("重复 ID 数量: {}", total_ids - unique_ids.len());

    if unique_ids.len() < total_ids {
        println!(" 检测到重复 ID！这证明了 static mut 在多线程下是不安全的。");
    } else {
        println!("本次运行未检测到重复 ID（但不代表是安全的，竞争条件是随机的）");
    }
    println!();
}

/// 测试 AtomicU16 版本的 UniqueId
/// 这个版本在多线程环境下是安全的
fn test_safe_version() {
    println!("=== 测试 AtomicU16 版本（安全） ===\n");

    let num_threads = 4;
    let ids_per_thread = 1000;
    let all_ids = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let all_ids = Arc::clone(&all_ids);
            thread::spawn(move || {
                let mut local_ids = Vec::new();
                for _ in 0..ids_per_thread {
                    let id = safe_version::next_id_safe();
                    local_ids.push(id);
                }
                let mut ids = all_ids.lock().unwrap();
                ids.extend(local_ids);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let all_ids = all_ids.lock().unwrap();
    let total_ids = all_ids.len();
    let unique_ids = all_ids.iter().collect::<std::collections::HashSet<_>>();

    println!("总 ID 数量: {}", total_ids);
    println!("唯一 ID 数量: {}", unique_ids.len());
    println!("重复 ID 数量: {}", total_ids - unique_ids.len());

    if unique_ids.len() == total_ids {
        println!("所有 ID 都是唯一的！AtomicU16 保证了线程安全。");
    } else {
        println!("这不应该发生，AtomicU16 应该保证线程安全。");
    }
    println!();
}

/// 演示 UniqueId 结构体的使用
fn demo_unique_id_struct() {
    println!("=== UniqueId 结构体演示 ===\n");

    /// 使用 AtomicU16 实现的线程安全唯一标识符
    #[derive(Debug, PartialEq, Eq)]
    struct UniqueId(u16);

    impl UniqueId {
        fn new() -> Self {
            static COUNTER: AtomicU16 = AtomicU16::new(0);
            let id = COUNTER.fetch_add(1, Ordering::SeqCst);
            UniqueId(id)
        }
    }

    let id1 = UniqueId::new();
    let id2 = UniqueId::new();
    let id3 = UniqueId::new();

    println!("UniqueId 1: {:?}", id1);
    println!("UniqueId 2: {:?}", id2);
    println!("UniqueId 3: {:?}", id3);

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);

    println!("\n✅ 所有 UniqueId 都是唯一的！\n");
}

/// 关于 Rust 中 unsafe 的讨论
fn discuss_unsafe() {
    println!("=== 关于 Rust 中 unsafe 的看法 ===\n");

    println!("Rust 的 unsafe 关键字是语言设计中的一个重要权衡：\n");

    println!("1. unsafe 存在的意义：");
    println!("   - 允许进行底层系统编程（如操作系统内核、驱动程序）");
    println!("   - 与 C/C++ 代码进行互操作");
    println!("   - 实现标准库中的基础数据结构");
    println!("   - 在性能关键路径上绕过安全检查\n");

    println!("2. unsafe 的风险：");
    println!("   - 绕过 Rust 的内存安全保证");
    println!("   - 可能导致数据竞争、悬垂指针、内存泄漏");
    println!("   - 未定义行为可能在不同编译器版本间变化");
    println!("   - Bug 更难调试和复现\n");

    println!("3. 最佳实践：");
    println!("   - 尽量缩小 unsafe 块的范围");
    println!("   - 为 unsafe 代码提供安全的抽象接口");
    println!("   - 使用文档注释说明 unsafe 的安全性条件");
    println!("   - 考虑使用 Miri 等工具检测未定义行为\n");

    println!("4. 本实验的启示：");
    println!("   - static mut 在多线程下是不安全的，因为：");
    println!("     * 多个线程可能同时读取和修改，导致数据竞争");
    println!("     * 编译器可能进行优化，导致不可预期的行为");
    println!("   - AtomicU16 提供了安全的替代方案：");
    println!("     * 使用 CPU 的原子指令保证操作的原子性");
    println!("     * 通过内存排序（Ordering）控制可见性");
    println!("     * 无需使用 unsafe 块\n");

    println!("5. 结论：");
    println!("   unsafe 是 Rust 安全模型的\"逃生舱\"，但应该谨慎使用。");
    println!("   在可能的情况下，优先选择安全的替代方案。\n");
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       UniqueId 线程安全性验证实验                         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // 测试不安全版本
    test_unsafe_version();

    // 测试安全版本
    test_safe_version();

    // 演示 UniqueId 结构体
    demo_unique_id_struct();

    // 讨论 unsafe
    discuss_unsafe();

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                    实验结束                               ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_version_uniqueness() {
        let num_threads = 10;
        let ids_per_thread = 100;
        let all_ids = Arc::new(Mutex::new(Vec::new()));

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let all_ids = Arc::clone(&all_ids);
                thread::spawn(move || {
                    let mut local_ids = Vec::new();
                    for _ in 0..ids_per_thread {
                        local_ids.push(safe_version::next_id_safe());
                    }
                    let mut ids = all_ids.lock().unwrap();
                    ids.extend(local_ids);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let all_ids = all_ids.lock().unwrap();
        let unique_count = all_ids.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, num_threads * ids_per_thread);
    }
}