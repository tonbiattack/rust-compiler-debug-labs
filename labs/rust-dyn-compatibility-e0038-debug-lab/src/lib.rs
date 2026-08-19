pub trait Snapshot {
    fn id(&self) -> String;
    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

pub struct Note {
    pub id: String,
}

impl Snapshot for Note {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn duplicate(&self) -> Self {
        Self {
            id: self.id.clone(),
        }
    }
}

pub fn render(snapshot: &dyn Snapshot) -> String {
    snapshot.id()
}
