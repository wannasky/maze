use std::hash::{Hash, Hasher};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize)]
pub struct Edge {
    // 开始坐标
    start: [i32; 2],
    // 结束坐标
    end: [i32; 2],
    // 共享边的节点，一个边一定有2个节点（外框不算）
    belong: Vec<i32>
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (start, end) = if self.start <= self.end {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        };
        start.hash(state);
        end.hash(state);
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}

impl Eq for Edge {}

impl Edge {

    pub fn new (start: [i32; 2], end: [i32; 2], belong: Vec<i32>) -> Self {

        Edge {
            start,
            end,
            belong
        }
    }

    pub fn get_belong(&self) -> &Vec<i32> {
        &self.belong
    }

    pub fn get_start(&self) -> [i32; 2] {
        self.start
    }

    pub fn get_end(&self) -> [i32; 2] {
        self.end
    }
}