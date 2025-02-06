use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

use compute::export::{
    egui::{emath::Numeric, DragValue, Ui},
    nalgebra::Vector3,
};

pub fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub fn dragger<Num: Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut Num,
    func: fn(DragValue) -> DragValue,
) {
    ui.horizontal(|ui| {
        ui.add(func(DragValue::new(value)));
        ui.label(label);
    });
}

pub fn vec3_dragger<Num: Numeric>(
    ui: &mut Ui,
    val: &mut Vector3<Num>,
    func: fn(DragValue) -> DragValue,
) {
    ui.horizontal(|ui| {
        ui.add(func(DragValue::new(&mut val[0])));
        ui.label("×");
        ui.add(func(DragValue::new(&mut val[1])));
        ui.label("×");
        ui.add(func(DragValue::new(&mut val[2])));
    });
}

pub fn hash<T: Hash>(item: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}
