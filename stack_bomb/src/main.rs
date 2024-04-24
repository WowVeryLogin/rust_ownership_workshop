const OBJECT_SIZE: usize = 1024 * 4;

trait Access {
    fn access(&mut self);
}

struct LargeObject([u8; OBJECT_SIZE]);

impl Access for LargeObject {
    fn access(&mut self) {
        self.0[0] = 1;
    }
}

struct LargeObjectRef<'a>(&'a mut [u8; OBJECT_SIZE]);

impl Access for LargeObjectRef<'_> {
    fn access(&mut self) {
        self.0[0] = 1;
    }
}

fn recursive_call<T: Access>(obj: T, i: usize) -> T {
    if i > 1024 {
        return obj;
    }
    recursive_call(obj, i + 1)
}

fn main() {
    let mut data = [0u8; OBJECT_SIZE];
    let obj = LargeObject(data);
    // let obj = LargeObjectRef(&mut data);
    let s = std::time::Instant::now();
    recursive_call(obj, 0);
    println!("{} us", s.elapsed().as_micros());
}

//objdump -Da target/debug/stack_bomb
//compare:
// 100000ce0: d14007ff    	sub	sp, sp, #1, lsl #12     ; =4096
// 100000ce4: d10143ff    	sub	sp, sp, #80
// 100000ce8: d100a7a9    	sub	x9, x29, #41
// 100000cec: f9000be9    	str	x9, [sp, #16]
// 100000cf0: aa0803e2    	mov	x2, x8
// 100000cf4: f9400be8    	ldr	x8, [sp, #16]
// 100000cf8: f9000fe2    	str	x2, [sp, #24]
// 100000cfc: f90013e0    	str	x0, [sp, #32]
// 100000d00: f90017e1    	str	x1, [sp, #40]
// 100000d04: f8001101    	stur	x1, [x8, #1]
// 100000d08: 3900011f    	strb	wzr, [x8]

// and 

// 100000d3c: f9000be0    	str	x0, [sp, #16]
// 100000d40: f9000fe1    	str	x1, [sp, #24]
// 100000d44: f81e03a0    	stur	x0, [x29, #-32]
// 100000d48: f81e83a1    	stur	x1, [x29, #-24]
// 100000d4c: 381df3bf    	sturb	wzr, [x29, #-33]
