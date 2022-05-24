use std::sync::Arc;

pub struct Obj {
    links: Vec<Arc<Obj>>
}

impl Obj {
    pub fn new() -> Self {
        Obj { links: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::Obj;

    #[test]
    fn directly_self_referencing() {
        let mut o = Obj::new();
        // Can't:
        //o.links.push(Arc::new(o));
    }

    #[test]
    fn directly_self_referencing_put_in_arc_first() {
        let mut o = Obj::new();
        let mut arc = Arc::new(o);
        // o is moved into arc, so we can't modify 'o' anymore

        let arc_clone = arc.clone();

        let as_ref = arc.as_ref();
        // Can't mutate arc, neither as_ref.
        
        // let try_into = arc.try_into(); // not supported/unknown type
        // let into: Obj = arc.into(); // not supported
    }
}
