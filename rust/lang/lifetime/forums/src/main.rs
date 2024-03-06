pub struct Forum<'a> {
    pub anons: Vec<&'a Anon<'a>>,
}
impl<'f> Forum<'f> {
    pub fn new() -> Forum<'static> {
        Forum { anons: vec![] }
    }

    pub fn add_user(&'f mut self, anon: &'f Anon) {
        self.anons.push(anon);
    }

    pub fn broadcast(&self, source: String, msg: String) {
        for anon in self.anons.iter() {
            if anon.id != source {
                anon.receive_msg(msg.clone());
            }
        }
    }
}

pub struct Anon<'a> {
    pub id: String,
    pub display: String,

    pub forum: Option<&'a Forum<'a>>,
}
impl<'a> Anon<'a> {
    pub fn new(display: String) -> Anon<'static> {
        Anon {
            display,
            forum: None,
            id: "testuuid".to_string(),
        }
    }

    pub fn join_forum(&'a mut self, forum: &'a mut Forum<'a>) {
        forum.add_user(self);
    }

    pub fn send_message(&self, msg: String) {
        if self.forum.is_some() {
            self.forum.as_ref().unwrap().broadcast(self.id.clone(), msg);
        }
    }

    pub fn receive_msg(&self, msg: String) {
        println!("Received {} from anon!", msg);
    }
}

fn main() {
    println!("Hello, world!");
}
