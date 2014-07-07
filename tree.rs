use std::kinds::marker::ContravariantLifetime;
use std::mem;

pub struct Tree<T> {
    root: *mut Node<T>,
}

struct Node<T> {
    parent: Option<*mut Node<T>>,
    first_child: Option<*mut Node<T>>,
    last_child: Option<*mut Node<T>>,
    prev_sibling: Option<*mut Node<T>>,
    next_sibling: Option<*mut Node<T>>,
    data: T,
}

impl<T> Node<T> {
    unsafe fn to_ref<'a>(&self) -> NodeRef<'a,T> {
        mem::transmute(NodeRef {
            ptr: mem::transmute::<_,*mut Node<T>>(self),
            lt: ContravariantLifetime,
        })
    }
}

pub struct NodeRef<'a,T> {
    ptr: *mut Node<T>,
    lt: ContravariantLifetime<'a>,
}

impl<'a,T> Deref<T> for NodeRef<'a,T> {
    fn deref<'b>(&'b self) -> &'b T {
        unsafe {
            &(*self.ptr).data
        }
    }
}

impl<'a,T> DerefMut<T> for NodeRef<'a,T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        unsafe {
            &mut (*self.ptr).data
        }
    }
}

impl<'a,T> NodeRef<'a,T> {
    pub fn parent(self) -> Option<NodeRef<'a,T>> {
        unsafe {
            (*self.ptr).parent.map(|p| (*p).to_ref())
        }
    }

    pub fn first_child(self) -> Option<NodeRef<'a,T>> {
        unsafe {
            (*self.ptr).first_child.map(|p| (*p).to_ref())
        }
    }

    pub fn last_child(self) -> Option<NodeRef<'a,T>> {
        unsafe {
            (*self.ptr).last_child.map(|p| (*p).to_ref())
        }
    }

    pub fn prev_sibling(self) -> Option<NodeRef<'a,T>> {
        unsafe {
            (*self.ptr).prev_sibling.map(|p| (*p).to_ref())
        }
    }

    pub fn next_sibling(self) -> Option<NodeRef<'a,T>> {
        unsafe {
            (*self.ptr).next_sibling.map(|p| (*p).to_ref())
        }
    }

    pub fn append_child(&mut self, child: NodeRef<T>) {
        unsafe {
            assert!((*child.ptr).parent.is_none());

            match (*self.ptr).last_child {
                None => (*self.ptr).first_child = Some(child.ptr),
                Some(last_child) => {
                    (*last_child).next_sibling = Some(child.ptr);
                    (*child.ptr).prev_sibling = Some(last_child);
                }
            }
            (*self.ptr).last_child = Some(child.ptr);
            (*child.ptr).parent = Some(self.ptr);
        }
    }
}

fn main() {}

