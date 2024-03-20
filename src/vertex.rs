use crate::id::Id;

use std::collections::{hash_map, HashMap};
use std::fmt::{self, Debug, Formatter};
use std::sync::{Arc, RwLock, RwLockReadGuard};

type Wrapper<T> = Arc<RwLock<Inner<T>>>;

#[derive(Clone)]
pub struct Vertex<T> {
    inner: Wrapper<T>,
}

impl<T> Vertex<T> {
    fn from_inner(inner: Wrapper<T>) -> Self {
        Self { inner }
    }

    /// Create vertex based on value
    pub fn new(value: T) -> Self {
        Self::from_inner(Arc::new(RwLock::new(Inner::new(value))))
    }

    /*
    /// Get a mutable reference to the inner value of the vertex
    pub fn get_mut(&self) -> &T {
        self.inner.read().unwrap().value
    }
    */

    /// Get vertex ID
    pub fn id(&self) -> Id {
        self.inner.read().unwrap().id
    }

    pub fn edges(&self) -> Edges<T> {
        let guard = self.inner.read().unwrap();
        Edges::from_guard(guard)
    }

    /// Connect both verticies to each other.
    pub fn link(&self, other: &Self) {
        self.connect(other);
        other.connect(self);
    }

    /// Connect vertex to other vertex
    pub fn connect(&self, other: &Self) {
        assert!(self.id() != other.id());
        self.inner
            .write()
            .unwrap()
            .edges
            .insert(other.id(), Arc::clone(&other.inner));
    }

    pub fn connects_to(&self, other: &Self) -> bool {
        self.inner.read().unwrap().edges.contains_key(&other.id())
    }

    pub fn connected_from(&self, other: &Self) -> bool {
        other.connects_to(self)
    }
}

impl<T: Copy> Vertex<T> {
    pub fn copy_inner(&self) -> T {
        self.inner.read().unwrap().value
    }
}

impl<T: Clone> Vertex<T> {
    pub fn clone_inner(&self) -> T {
        self.inner.read().unwrap().value.clone()
    }
}

impl<T: Debug> Debug for Vertex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let inner = self.inner.read().unwrap();
        Debug::fmt(&inner, f)
    }
}

pub struct Inner<T> {
    id: Id,
    value: T,
    edges: HashMap<Id, Arc<RwLock<Self>>>,
}

impl<T> Inner<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: Id::new(),
            value,
            edges: Default::default(),
        }
    }
}

impl<T: Debug> Debug for Inner<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let edges: Vec<_> = self.edges.keys().map(|id| id.to_string()).collect();

        f.debug_struct("Vertex")
            .field("id", &self.id.to_string())
            .field("value", &self.value)
            .field("edges", &edges)
            .finish()
    }
}

pub struct Edges<'a, T> {
    guard: RwLockReadGuard<'a, Inner<T>>,
    //values: &'a hash_map::Values<'a, Id, Wrapper<T>>,
}

impl<'a, T> Edges<'a, T> {
    fn from_guard(guard: RwLockReadGuard<'a, Inner<T>>) -> Self {
        //let guard = inner.read().unwrap();
        Self { guard }
        //Self { guard: &guard, values }
    }
}

impl<T> Iterator for Edges<'_, T> {
    type Item = Vertex<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next().map(Arc::clone).map(Vertex::from_inner)
    }
}
