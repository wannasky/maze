use std::collections::{HashSet};
use rand;
use rand::Rng;
use crate::edge::Edge;
use wasm_bindgen::prelude::*;

mod edge;

#[wasm_bindgen]
#[derive(Debug, Clone)]
struct Maze {
    columns: i32,
    rows: i32,
    size: i32,
    visited: Vec<i32>,
    edges: Vec<Edge>,
    remove_edges: Vec<Edge>,
}

#[wasm_bindgen]
impl Maze {
    #[wasm_bindgen(constructor)]
    pub fn new(columns: i32, rows: i32, size: i32) -> Self {
        Maze {
            columns,
            rows,
            size,
            visited: vec![],
            edges: vec![],
            remove_edges: vec![],
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self) {
        self.visited.push(0);
        self.extend_edges(0);
        self.generate();
    }

    #[wasm_bindgen]
    pub fn get_remove_edges(&self) -> JsValue {
        let remove_edges = self.remove_edges.clone();
        serde_wasm_bindgen::to_value(&remove_edges).unwrap()
    }


    pub fn generate(&mut self) {
        while self.edges.len() != 0 {
            // 从中随机挑选一个墙
            let pick_index = self.pick_edge() as usize;
            let edge = self.edges.get(pick_index).unwrap();
            // 获取没有访问过的节点，如果已经访问，此墙不拆
            if let Some(next_cell) = self.get_next_cell(edge) {
                // 放入已访问列表
                self.visited.push(next_cell);
                // 开始标记此墙已拆
                self.remove_edges.push(edge.clone());
                // 拓展edges
                self.extend_edges(next_cell);
            }
            self.edges.remove(pick_index);
        }
    }

    pub fn extend_edges(&mut self, index: i32) {
        self.edges.extend(self.find_cell_edge(index));
        let mut set = HashSet::new();
        // 去重
        self.edges.retain(|edge| set.insert(edge.clone()));
    }

    pub fn get_next_cell(&self, edge: &Edge) -> Option<i32> {
        let belong = edge.get_belong();
        let remain: Vec<i32> = belong.iter().filter(|&&cell| !self.visited.contains(&cell)).cloned().collect();
        remain.get(0).cloned()
    }

    pub fn pick_edge(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.edges.len()) as i32
    }

    pub fn find_cell_edge(&self, index: i32) -> Vec<Edge> {
        let mut edges = Vec::new();
        let vertex_coord = self.get_cell_vertex_coord(index);
        let &vc_x = vertex_coord.get(0).unwrap();
        let &vc_y = vertex_coord.get(1).unwrap();
        let top_index = index - self.columns;
        let right_index = index + 1;
        let bottom_index = index + self.columns;
        let left_index = index - 1;

        // top_index < 0 时没有top节点
        if top_index >= 0 {
            edges.push(Edge::new(
                [vc_x, vc_y],
                [vc_x + self.size, vc_y],
                vec![index, top_index],
            ));
        }
        // 最右侧列没有right节点
        if (index + 1) % self.columns != 0 {
            edges.push(Edge::new(
                [vc_x + self.size, vc_y],
                [vc_x + self.size, vc_y + self.size],
                vec![index, right_index],
            ));
        }
        // 最下没有bottom节点
        if bottom_index < self.columns * self.rows {
            edges.push(Edge::new(
                [vc_x + self.size, vc_y + self.size],
                [vc_x, vc_y + self.size],
                vec![index, bottom_index],
            ));
        }
        // 最左侧没有left节点
        if index % self.columns != 0 {
            edges.push(Edge::new(
                [vc_x, vc_y + self.size],
                [vc_x, vc_y],
                vec![index, left_index],
            ));
        }
        // 去除已经标记为删除的边
        edges.retain(|edge| !self.edge_is_remove(edge));
        edges
    }

    pub fn edge_is_remove(&self, edge: &Edge) -> bool {
        self.remove_edges.iter().any(|item| item == edge)
    }

    pub fn get_cell_vertex_coord(&self, index: i32) -> Vec<i32> {
        let column = index % self.columns;
        let row = index / self.columns;
        vec![column * self.size, row * self.size]
    }
}

#[cfg(test)]
mod tests {
    use crate::Maze;

    #[test]
    pub fn test() {
        let mut maze = Maze::new(100, 100, 10);
        maze.init();
        println!("{:?}", maze);
    }
}
