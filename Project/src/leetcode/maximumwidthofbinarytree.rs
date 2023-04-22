use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cmp;
use crate::models::treenode::TreeNode;

// mod models;
trait Solution662 {}

impl dyn Solution662 {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let first = root.unwrap();
        // let mut node = RefCell::borrow_mut(&first);
        // node.val=3;
        // let muu=first.borrow_mut();
        // let node2 = RefCell::borrow_mut(Deref::deref(&first));//first.borrow();

        let mut result = 0;
        let mut bfs: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let node = Rc::clone(root.as_ref().unwrap());
        RefCell::borrow_mut(&node).val=1;
        bfs.push_back(node);
        loop{
            let len = bfs.len();
            let mut leftval = 0;
            let mut rightval=0;
            println!("{}",len);
            for _ in 0..len{
                let treenode = Rc::clone(&bfs.pop_front().unwrap());
                if leftval==0{
                    leftval = RefCell::borrow(&treenode).val;
                }
                rightval=RefCell::borrow(&treenode).val;

                
                if RefCell::borrow(&treenode).left.is_none(){
                    let left = Rc::clone(RefCell::borrow(&treenode).left.as_ref().unwrap());
                //if !Some(Rc::clone(&left)).is_none(){
                    RefCell::borrow_mut(&left).val=RefCell::borrow(&treenode).val*2;
                    bfs.push_back(left);
                }

                if !RefCell::borrow(&treenode).right.is_none(){
                    let right = Rc::clone(RefCell::borrow(&treenode).right.as_ref().unwrap());
                    RefCell::borrow_mut(&right).val=RefCell::borrow(&treenode).val*2;
                    bfs.push_back(right);
                }

                result = cmp::max(result,rightval-leftval);
            }
            if bfs.len()<=0{
                break;
            }
        }

        // if let Some(r) = root {
        //     let first = Rc::clone(&root.unwrap());
        //     RefCell::borrow_mut(&first).val=3;
            
        // }
        // let first = Rc::clone(root.as_ref().unwrap());
        // println!("{}",RefCell::borrow_mut(&first).val);
        // let third=Rc::clone(root.as_ref().unwrap());
        // RefCell::borrow_mut(&third).val=30;
        // println!("{}",RefCell::borrow_mut(&first).val);
        // let forth=Rc::clone(&root.as_ref().unwrap());
        // let second=Rc::clone(&root.unwrap());
        // let fifth=Rc::clone(&root.unwrap());
        // println!("{}",first.val);
        // bfs.push_back(first);

        // while !bfs.is_empty(){
        //     let len = bfs.len();
        //     for _ in 0..len{
        //         let mut first = bfs.pop_front().unwrap();

        //         if first.into_inner().left.is_none(){
        //             let mut left =first.into_inner().left;
        //             left.unwrap().into_inner().val=first.into_inner().val*2;

        //             bfs.push_back(left.unwrap());
        //         }
        //         if first.into_inner().left.is_none(){
        //             let mut right =first.into_inner().right;

        //             bfs.push_back(right.unwrap());
        //         }
        //     }
        // }

        result as i32
    }
}
