use gc::*;
use scope::*;
use std::thread;
use std::time::Duration;

pub fn spawn_gc(ctx: &Context) {
    let new_roots = ctx.state.roots.clone();
    let new_alloc = ctx.alloc.clone();
    thread::spawn(move || {
        loop {
            {
                //println!("roots check: {:?}", new_roots.len());
                println!("alloc check: {:?}", new_alloc.read().unwrap().size());
            }

            {
                let mut arena = new_alloc.write().unwrap();
                arena.reset();
                let len = new_roots.len();
                for i in 0..len {
                    new_roots.get(i, |value| {
                        GcArena::mark(value);
                    });
                }
                arena.sweep();
            }

            thread::sleep(Duration::from_millis(10));
            //for i in 0..new_alloc.len() {
            //    new_alloc.get(i, |v| {
            //        println!("root {:?}", v);
            //    });
            //}
            //new_roots.inner.each(|k, v| {
            //    println!("key: {:?}", k);
            //})
        }
    });
}
