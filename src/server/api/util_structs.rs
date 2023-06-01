use crate::server::api::query_types::Query;

pub struct PathSegment {
    pub value: String,
    pub seg_number: u16,
    pub children_segments: Vec<Box<PathSegment>>,
    pub query: Option<Query>,
}

impl PathSegment {
    pub fn new(value: String, seg_number: u16) -> PathSegment {
        PathSegment {
            value,
            seg_number,
            children_segments: Vec::new(),
            query: None,
        }
    }

    pub fn child_seg(&mut self, child: PathSegment) {

        self.children_segments.push(Box::new(child));
    }

    pub fn child_seg_by_value(&mut self, value: String) -> &mut Box<PathSegment> {
     
       
        let child = PathSegment::new(value.clone(), self.seg_number + 1);
        self.children_segments.push(Box::new(child));
        self.children_segments.last_mut().unwrap()
    }

    pub fn get_next(&self, seg_value: String) -> Option<&Box<PathSegment>> {
        if self.children_segments.len() == 0 {
            return None;
        }

        for child in &self.children_segments {
       
            if child.value.eq(&seg_value) {
                return Some(child);
            }
        }
        None
    }

    pub fn has_children(&self) -> bool {
        self.children_segments.len() > 0
    }
}
