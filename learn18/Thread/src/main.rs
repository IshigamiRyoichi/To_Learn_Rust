use std::thread;
use std::time::Duration;
use std::io::Read;
use futures::executor::ThreadPool;

fn test1() {
    let handle = thread::spawn(||{
        for i in 0..10{
            println!("thread #1 count {}.",i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("press enter key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}

fn test2() {
    let handle = thread::spawn(||{
        for i in 0..10{
            println!("thread #1 count {}.",i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("press wait.");
    handle.join().unwrap();
    println!("program end.");
}

fn test3() {
    let handle = thread::spawn(|| {
        for i in 0..10{
            println!("thread #1 count {}.",i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 0..10{
            println!("thread #2 count {}.",i);
            thread::sleep(Duration::from_millis(2000));
        }
    });
    let handle3 = thread::spawn(|| {
        for i in 0..10{
            println!("thread #3 count {}.",i);
            thread::sleep(Duration::from_millis(1500));
        }
    });
    println!("press wait.");
    handle.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    println!("program end.");

}

fn test4() {
    let task = || {
        for i in 0..10{
            println!("thread #1 count {}.",1);
            thread::sleep(Duration::from_millis(1000));
        }
    };
    let handle = thread::spawn(task);
    println!("please wait.");
    handle.join().unwrap();
    println!("program end");
}

fn test5() {
    fn task() {
        for i in 0..10{
            println!("thread #1 count {}.",1);
            thread::sleep(Duration::from_millis(1000));
        }
    };
    let handle = thread::spawn(task);
    println!("please wait.");
    handle.join().unwrap();
    println!("program end");
}

fn foo(id: i32){
    for i in 0..10{
        println!("thread #{} count {}.",id,1);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn test6() {
    println!("program star.");
    let h1 = thread::spawn(|| {foo(10);});
    let h2 = thread::spawn(|| {foo(20);});
    let h3 = thread::spawn(|| {foo(30);});
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    println!("program end");
}
fn test7() {
    println!("program star.");
    let h1 = thread::spawn(|| {foo(10);}).join().unwrap();
    let h2 = thread::spawn(|| {foo(20);}).join().unwrap();
    let h3 = thread::spawn(|| {foo(30);}).join().unwrap();
    println!("program end");
}

async fn foo2(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}.",id,i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn test8() {
    let task = async {
        foo2(10).await;
        foo2(20).await;
        foo2(30).await;
    };
    println!("program start.");
    futures::executor::block_on(task);
    println!("program end.");
}

async fn sub() {
    foo2(10).await;
    foo2(20).await;
    foo2(30).await;
}

fn test9() {
    println!("program start.");
    futures::executor::block_on(sub());
    println!("program end.");
}

async fn foo3(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

#[tokio::main]
async fn test10() {
    println!("program start.");
    foo3(10).await;
    foo3(20).await;
    foo3(30).await;
    println!("program end.");
}

fn test11() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    println!("program start.");
    rt.block_on(async {
        foo3(10).await;
        foo3(20).await;
        foo3(30).await;
    });
    println!("program end.");
}

fn test12(){
    let pool = ThreadPool::new().unwrap();
    let task = async {
        for j in 1..6 {
            // let id = j * 10;
            let mut id = 0;
            pool.spawn_ok(async move {
                for i in 0..10 {
                    println!("thread #{} count {}.",id, i);
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            thread::sleep(Duration::from_millis(500));   
        }
    };
    println!("program start");
    futures::executor::block_on(task);
    println!("press any key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}

fn main() {
    test12();
}