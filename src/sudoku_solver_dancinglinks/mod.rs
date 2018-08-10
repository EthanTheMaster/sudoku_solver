use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Constraint {
    //First u8 is the row/column/block number
    //Second u8 is the value occupying that row/column/block
    ROW(u8, u8),
    COL(u8, u8),
    BLOCK(u8, u8),

    //Two numbers cannot be in the same position
    POS(u8),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Operation {
    pub x_pos: u8,
    pub y_pos: u8,
    pub value: u8,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum NodeType {
    //A column header is represented by a constraint and size value
    ColumnHeader(Constraint, usize),
    //A field is represented by its corresponding action and its corresponding column header [id]
    Field(Operation, usize)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub id: usize,
    pub left_id: usize,
    pub right_id: usize,
    pub up_id: usize,
    pub down_id: usize
}

pub struct Network {
    network: Vec<RefCell<Node>>,
    current_id: usize,
}

pub struct HorizontalNodeIterator<'a> {
    network: &'a Network,
    start_node: usize,
    current_node: usize,
    has_started: bool,
}

pub struct VerticalNodeIterator<'a> {
    network: &'a Network,
    start_node: usize,
    current_node: usize,
    has_started: bool,
}

impl Operation {
    pub fn new(x_pos: u8, y_pos: u8, value: u8) -> Operation {
        Operation {
            x_pos,
            y_pos,
            value,
        }
    }
}

impl Network {
    pub fn new() -> Network {
        Network {
            network: Vec::new(),
            current_id: 0,
        }
    }

    //Makes a node for the network and returns the id for the newly created node
    pub fn create_node(&mut self, node_type: NodeType) -> usize {
        let node_id = self.current_id;
        let node = Node {
            node_type: node_type.clone(),
            id: node_id,
            left_id: node_id,
            right_id: node_id,
            up_id: node_id,
            down_id: node_id,
        };

        self.current_id += 1;
        self.network.push(RefCell::new(node));

        //After adding the node to the network, check if the node is a field and append it under
        //its corresponding column header
        if let NodeType::Field(operation, column_header_id) = node_type {
            self.append_node_vertically(column_header_id, node_id);
            //Increment the size
            match self.get_node(column_header_id).borrow_mut().node_type {
                NodeType::ColumnHeader(ref mut constraint, ref mut size) => {
                    *size += 1;
                },
                _ => {panic!()},
            }
        }

        return node_id;
    }

    pub fn get_node(&self, node_id: usize) -> &RefCell<Node> {
        self.network.get(node_id).unwrap()
    }

    pub fn get_column_header(&self, node_id: usize) -> &RefCell<Node> {
        match self.get_node(node_id).borrow().node_type {
            NodeType::ColumnHeader(_, _) => {
                panic!("Node is not a field!");
            },
            NodeType::Field(operation, column_header_id) => {
                return self.get_node(column_header_id);
            },
        }
    }

    pub fn get_column_header_size(&self, header_id: usize) -> usize {
        match self.get_node(header_id).borrow().node_type {
            NodeType::ColumnHeader(constraint, size) => {
                return size;
            },
            NodeType::Field(_, _) => {
                panic!("Node is not a column header")
            },
        }
    }

    //Append node2 to the right of node1
    pub fn append_node_horizontally(&self, node1_id: usize, node2_id: usize) {
        //Can't append the same node to itself
        assert_ne!(node1_id, node2_id);

        let node1 = self.get_node(node1_id);
        let node2 = self.get_node(node2_id);
        let node1_right = self.get_node(node1.borrow().right_id);

        node2.borrow_mut().left_id = node1_id;
        node2.borrow_mut().right_id = node1_right.borrow().id;

        node1.borrow_mut().right_id = node2_id;
        node1_right.borrow_mut().left_id = node2_id;
    }

    //Append node2 to the bottom of node1
    pub fn append_node_vertically(&self, node1_id: usize, node2_id: usize) {
        //Can't append the same node to itself
        assert_ne!(node1_id, node2_id);

        let node1 = self.get_node(node1_id);
        let node2 = self.get_node(node2_id);
        let node1_down = self.get_node(node1.borrow().down_id);

        node2.borrow_mut().up_id = node1_id;
        node2.borrow_mut().down_id = node1_down.borrow().id;

        node1.borrow_mut().down_id = node2_id;
        node1_down.borrow_mut().up_id = node2_id;
    }

    pub fn horizontal_iter(&self, node_id: usize) -> HorizontalNodeIterator {
        HorizontalNodeIterator {
            network: &self,
            start_node: node_id,
            current_node: node_id,
            has_started: false,
        }
    }

    pub fn vertical_iter(&self, node_id: usize) -> VerticalNodeIterator {
        VerticalNodeIterator {
            network: &self,
            start_node: node_id,
            current_node: node_id,
            has_started: false,
        }
    }

    pub fn cover_column(&self, column_id: usize) {
        if let NodeType::Field(_, _) = self.get_node(column_id).borrow().node_type {
            panic!("Not a column!");
        }
        let current_header = self.get_node(column_id);

        let right_id = current_header.borrow().right_id;
        let left_id = current_header.borrow().left_id;

        let right_header = self.get_node(right_id);
        let left_header = self.get_node(left_id);

        //Cover the column header
        right_header.borrow_mut().left_id = left_id;
        left_header.borrow_mut().right_id = right_id;

        let mut column_iterator = self.vertical_iter(column_id);
        //Omit the column header from the iteration
        column_iterator.next();
        for col_field_id in column_iterator {
            let mut row_iterator = self.horizontal_iter(col_field_id);
            //Omit the starting field
            row_iterator.next();
            for row_field_id in row_iterator {
                //Cover up the current field
                let column_header = self.get_column_header(row_field_id);

                let current_node = self.get_node(row_field_id);

                let above_id = current_node.borrow().up_id;
                let down_id = current_node.borrow().down_id;

                let above_node = self.get_node(above_id);
                let down_node = self.get_node(down_id);

                above_node.borrow_mut().down_id = down_id;
                down_node.borrow_mut().up_id = above_id;

                //Lower the size of the current column header
                match column_header.borrow_mut().node_type {
                    NodeType::ColumnHeader(constraint, ref mut size) => {
                        *size -= 1;
                    },
                    _ => {panic!()},
                }
            }
        }
    }

    pub fn uncover_column(&self, column_id: usize) {
        if let NodeType::Field(_, _) = self.get_node(column_id).borrow().node_type {
            panic!("Not a column!");
        }
        let current_header = self.get_node(column_id);

        let right_header = self.get_node(current_header.borrow().right_id);
        let left_header = self.get_node(current_header.borrow().left_id);

        //Uncover the column header
        right_header.borrow_mut().left_id = column_id;
        left_header.borrow_mut().right_id = column_id;

        let mut column_iterator = self.vertical_iter(column_id);
        //Omit the column header from the iteration
        column_iterator.next();
        for col_field_id in column_iterator {
            let mut row_iterator = self.horizontal_iter(col_field_id);
            //Omit the starting field
            row_iterator.next();
            for row_field_id in row_iterator {
                //Uncover up the current field
                let column_header = self.get_column_header(row_field_id);

                let current_node = self.get_node(row_field_id);

                let above_node = self.get_node(current_node.borrow().up_id);
                let down_node = self.get_node(current_node.borrow().down_id);

                above_node.borrow_mut().down_id = row_field_id;
                down_node.borrow_mut().up_id = row_field_id;

                //Lower the size of the current column header
                match column_header.borrow_mut().node_type {
                    NodeType::ColumnHeader(constraint, ref mut size) => {
                        *size += 1;
                    },
                    _ => {panic!()},
                }
            }
        }
    }

    pub fn solve_exact_cover(&self, column_header_root_id: usize, solution: &mut Vec<Operation>) -> bool {
        //Find column header with smallest size
        let root = self.get_node(column_header_root_id);
        let mut smallest_id = root.borrow().right_id;

        let mut headers_iter = self.horizontal_iter(column_header_root_id);
        //Omit the root header
        headers_iter.next();
        for header_id in headers_iter {
            if self.get_column_header_size(header_id) < self.get_column_header_size(smallest_id) {
                smallest_id = header_id;
            }
        }

        //Solution has been found! ... Empty matrix
        if smallest_id == column_header_root_id {
            return true
        }
        //Impossible configuration to solve
        if self.get_column_header_size(smallest_id) == 0 {
            return false;
        }

        //Cover the smallest header
        self.cover_column(smallest_id);

        let mut col_iter = self.vertical_iter(smallest_id);
        col_iter.next();

        for col_field_id in col_iter {
            //Cover the row
            let mut row_iter = self.horizontal_iter(col_field_id);
            row_iter.next();

            for row_field_id in row_iter {
                let current_node_header_id = self.get_column_header(row_field_id).borrow().id;
                self.cover_column(current_node_header_id);
            }

            let current_operation = match self.get_node(col_field_id).borrow().node_type {
                NodeType::ColumnHeader(_, _) => {panic!()},
                NodeType::Field(operation, _) => {
                    operation.clone()
                },
            };
            //Add current row to the partial solution
            solution.push(current_operation);
            if self.solve_exact_cover(column_header_root_id, solution) {
                return true;
            }
            //If no solution was found, backtrack...
            solution.pop();

            //Uncover the row
            let mut row_iter = self.horizontal_iter(col_field_id);
            row_iter.next();

            for row_field_id in row_iter {
                let current_node_header_id = self.get_column_header(row_field_id).borrow().id;
                self.uncover_column(current_node_header_id);
            }
        }

        self.uncover_column(smallest_id);

        return false;
    }
}

impl<'a> Iterator for HorizontalNodeIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current_node_id = self.current_node;
        let current_node = self.network.get_node(current_node_id);
        let next_node_id = current_node.borrow().right_id;

        if !self.has_started {
            self.has_started = true;
            return Some(current_node_id);
        }

        self.current_node = next_node_id;
        if next_node_id == self.start_node {
            return None;
        }

        return Some(next_node_id);
    }
}

impl<'a> Iterator for VerticalNodeIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current_node_id = self.current_node;
        let current_node = self.network.get_node(current_node_id);
        let next_node_id = current_node.borrow().down_id;

        if !self.has_started {
            self.has_started = true;
            return Some(current_node_id);
        }

        self.current_node = next_node_id;
        if next_node_id == self.start_node {
            return None;
        }

        return Some(next_node_id);
    }
}

pub fn solve_sudoku(board_string: &str) -> Vec<u8> {
    assert_eq!(board_string.len(), 81, "Board is not correct size!");
    let mut board = Vec::with_capacity(81);

    //Parse the board_string and compute all constraints already fulfilled with the given input
    let mut fulfilled_constraints: HashSet<Constraint> = HashSet::new();
    for (idx, character) in board_string.chars().enumerate() {
        let row = (idx / 9) as u8;
        let col = (idx % 9) as u8;
        let block = row / 3 * 3 + col / 3;

        match character.to_digit(10) {
            Some(n) => {
                let val = n as u8;
                board.push(val);

                if val != 0 {
                    fulfilled_constraints.insert(Constraint::ROW(row, val));
                    fulfilled_constraints.insert(Constraint::COL(col, val));
                    fulfilled_constraints.insert(Constraint::BLOCK(block, val));
                }
            },
            None => {
                if character == '_' || character == '.' {
                    board.push(0);
                } else {
                    panic!("Invalid character at index: {}", idx);
                }
            }
        }
    }

    //Map a constraint to the corresponding column header id
    let mut constraints: HashMap<Constraint, usize> = HashMap::new();

    let mut network = Network::new();
    let column_header_root = network.create_node(NodeType::ColumnHeader(Constraint::ROW(255,255), 0));

    for (idx, val) in board.iter().enumerate() {
        //Compute all possible values for the blank space
        if *val == 0 {
            let row = (idx / 9) as u8;
            let col = (idx % 9) as u8;
            let block = row / 3 * 3 + col / 3;

            for possible_val in 1..10 {
                let row_constraint = Constraint::ROW(row, possible_val);
                let col_constraint = Constraint::COL(col, possible_val);
                let block_constraint = Constraint::BLOCK(block, possible_val);
                let position_constraint = Constraint::POS(idx as u8);

                //Not a valid operation...
                if fulfilled_constraints.contains(&row_constraint) ||
                    fulfilled_constraints.contains(&col_constraint) ||
                    fulfilled_constraints.contains(&block_constraint) {
                    continue;
                }

                let current_operation = Operation::new(col, row, possible_val);

                //Get column header associated with the current row constraint
                if !constraints.contains_key(&row_constraint) {
                    //If this row constraint has never been seen before, add it to the network
                    let new_header_id = network.create_node(NodeType::ColumnHeader(row_constraint.clone(), 0));
                    network.append_node_horizontally(column_header_root, new_header_id);
                    constraints.insert(row_constraint.clone(), new_header_id);
                }
                let row_column_header_id = constraints.get(&row_constraint).unwrap().clone();

                //Get column header associated with the current column constraint
                if !constraints.contains_key(&col_constraint) {
                    //If this column constraint has never been seen before, add it to the network
                    let new_header_id = network.create_node(NodeType::ColumnHeader(col_constraint.clone(), 0));
                    network.append_node_horizontally(column_header_root, new_header_id);
                    constraints.insert(col_constraint.clone(), new_header_id);
                }
                let col_column_header_id = constraints.get(&col_constraint).unwrap().clone();

                //Get column header associated with the current block constraint
                if !constraints.contains_key(&block_constraint) {
                    //If this block constraint has never been seen before, add it to the network
                    let new_header_id = network.create_node(NodeType::ColumnHeader(block_constraint.clone(), 0));
                    network.append_node_horizontally(column_header_root, new_header_id);
                    constraints.insert(block_constraint.clone(), new_header_id);
                }
                let block_column_header_id = constraints.get(&block_constraint).unwrap().clone();

                //Get column header associated with the current position constraint
                if !constraints.contains_key(&position_constraint) {
                    //If this position constraint has never been seen before, add it to the network
                    let new_header_id = network.create_node(NodeType::ColumnHeader(position_constraint.clone(), 0));
                    network.append_node_horizontally(column_header_root, new_header_id);
                    constraints.insert(position_constraint.clone(), new_header_id);
                }
                let pos_column_header_id = constraints.get(&position_constraint).unwrap().clone();

                //"Inserting a row into the exact cover matrix" describing the current operation
                let field1 = network.create_node(NodeType::Field(current_operation.clone(), row_column_header_id));
                let field2 = network.create_node(NodeType::Field(current_operation.clone(), col_column_header_id));
                let field3 = network.create_node(NodeType::Field(current_operation.clone(), block_column_header_id));
                let field4 = network.create_node(NodeType::Field(current_operation.clone(), pos_column_header_id));

                network.append_node_horizontally(field1, field2);
                network.append_node_horizontally(field2, field3);
                network.append_node_horizontally(field3, field4);
            }
        }
    }

    let mut solution_set = Vec::new();
    network.solve_exact_cover(column_header_root, &mut solution_set);

    let mut result = board.clone();
    //Apply the operations to the board
    for operation in solution_set.iter() {
        *result.get_mut((operation.y_pos * 9 + operation.x_pos) as usize).unwrap() = operation.value;
    }

    return result;
}