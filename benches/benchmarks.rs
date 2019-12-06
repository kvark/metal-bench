#[macro_use]
extern crate criterion;
#[macro_use]
extern crate objc;

use cocoa::foundation::NSAutoreleasePool;
use metal::*;

fn device_ops(c: &mut criterion::Criterion) {
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

fn command_ops(c: &mut criterion::Criterion) {
    let pool = unsafe { NSAutoreleasePool::new(cocoa::base::nil) };

    let device = Device::system_default().expect("no device found");
    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    c.bench_function("MTLCommandBuffer::new_compute_command_encoder", |b| b.iter(|| {
        let encoder = command_buffer.new_compute_command_encoder();
        encoder.end_encoding();
    }));

    let rp_desc = RenderPassDescriptor::new();

    c.bench_function("MTLCommandBuffer::new_render_command_encoder", |b| b.iter(|| {
        let encoder = command_buffer.new_render_command_encoder(&rp_desc);
        encoder.end_encoding();
    }));

    unsafe {
        let () = msg_send![pool, release];
    }
}

criterion_group!(benches, device_ops, command_ops);
criterion_main!(benches);
