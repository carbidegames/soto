use {Geometry, Model};

#[derive(Debug, Clone)]
pub struct Object {
    //properties: ObjectProperties,
    /// Contains the type and type-specific data.
    pub class: ObjectType,
}

impl Object {
    pub fn new_root() -> Self {
        Object {
            class: ObjectType::Root,
        }
    }

    pub fn id(&self) -> i64 {
        match self.class {
            ObjectType::Geometry(ref g) => g.id,
            ObjectType::Model(ref m) => m.id,
            ObjectType::Root => 0,
            ObjectType::NotSupported(_) => -1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    Geometry(Geometry),
    Model(Model),
    /// Virtual object representing the root of the file.
    Root,
    /// Currently unsupported object type.
    NotSupported(String)
}
