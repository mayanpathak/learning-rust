// pub struct List<T>{
//     head: Link<T>,
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T>{
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List <T>{
//     pub fn new()-> Self{
//         List { head: None}

//     }

// pub fn push(&mut self, elem:T){
//     let new_node = Box::new(Node{
//         elem,
//         next: self.head.take(),
//     });

//     self.head = Some(new_node);

// }

// pub fn pop(&mut self)-> Option<T>{
//     self.head.take().map(|boxed_node|{
//         let node = *boxed_node;
//         self.head = node.next;
//         node.elem
//     })
// }

// pub fn peek(&self)-> Option<&T>{
//  self.head.as_ref().map(|node| &node.elem)}


//  pub fn peek_mut(&self) -> Option<&mut T> {
//     {
//         self.head.as_mut().map(|node| &mut node.elem)
//  }

//  pub fn iter(&self) -> Iter <'_, T>{
//     Iter {
//         next:self.head.as_deref(), 
//     }

//  }

//  pub fn iter_mut(&mut self) -> IterMut<'_, T>{
//     IterMut{
//         next:self.head.as_deref_mut(),
//     }
//  }




// }

// impl<T> Drop for List<T>
// {
//     fn drop (&mut self){
//         let mut cur_link = self.head.take();
//         while let some(mut boxed_node) = cur_link{
//             cur_link = boxed_node.next.take();

//         }
//     }
// }
//  pub struct Iter<'a, T>{
//     next : Option<&'a Node<T>>,
//  }

//  impl <'a,T> Iterator for Iter<'a,T>{
//     type Item = &'a T;

//     fn next(&mut self) -> Option<self::Item>{
//         self.next.map(|node|{
//             self.next = node.next.as_deref();
//             &node.elem
//         })
//     }
//  }

//  pub struct IterMut<'a, T> {
//     next: Option<&'a mut Node<T>>,
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         // take() leaves None and returns the previous Option<&mut Node<T>>
//         self.next.take().map(|node| {
//             self.next = node.next.as_deref_mut(); // Option<&mut Node<T>>
//             &mut node.elem
//         })
//     }
// }



// src/lib.rs

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // push: insert at front. `elem` is moved into the Node (ownership transferred).
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(), // take() moves the Option out, leaving None
        });
        self.head = Some(new_node); // Box<Node<T>> is moved into head
    }

    // pop: remove from front and return the owned element (T).
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = *boxed_node; // move Node<T> out of Box<Node<T>>
            self.head = node.next;  // reuse next as new head (move)
            node.elem               // return owned element
        })
    }

    // peek: borrow the front element immutably (&T)
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // peek_mut: borrow the front element mutably (&mut T)
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // iter: returns an iterator that yields &T
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(), // Option<&Node<T>>
        }
    }

    // iter_mut: returns an iterator that yields &mut T
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(), // Option<&mut Node<T>>
        }
    }
}

// Implement Drop to avoid recursive drop (iterative drop)
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            // Take the next link out of the node so boxed_node can be dropped
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope here and is dropped
        }
    }
}

// Immutable iterator (borrows the list)
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref(); // Option<&Node<T>>
            &node.elem
        })
    }
}

// Mutable iterator (borrows list mutably)
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // take() leaves None and returns the previous Option<&mut Node<T>>
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut(); // Option<&mut Node<T>>
            &mut node.elem
        })
    }
}