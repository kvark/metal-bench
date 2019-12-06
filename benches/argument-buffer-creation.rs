#[macro_use]
extern crate criterion;
#[macro_use]
extern crate objc;

use cocoa::foundation::NSAutoreleasePool;
use metal::*;

fn argbuf_create_benchmark(c: &mut criterion::Criterion) {
    let pool = unsafe { NSAutoreleasePool::new(cocoa::base::nil) };
    let device = Device::system_default().expect("no device found");

    let mut descriptors = Vec::new();
    for i in 0 .. 10 {
        let desc1 = ArgumentDescriptor::new();
        desc1.set_data_type(MTLDataType::Texture);
        desc1.set_index(i*3+0);
        descriptors.push(desc1);
        let desc2 = ArgumentDescriptor::new();
        desc2.set_data_type(MTLDataType::Sampler);
        desc2.set_index(i*3+1);
        descriptors.push(desc2);
        let desc3 = ArgumentDescriptor::new();
        desc3.set_data_type(MTLDataType::Pointer);
        desc3.set_index(i*3+2);
        descriptors.push(desc3);
    }

    c.bench_function("MTLDevice::new_argument_encoder", |b| b.iter(|| {
        let _ = device.new_argument_encoder(&Array::from_slice(&descriptors));
    }));
    unsafe {
        let () = msg_send![pool, release];
    }
}

criterion_group!(benches, argbuf_create_benchmark);
criterion_main!(benches);
