use crate::enumerate::encode_single_position;

pub struct Action {
    pub start_position: Option<usize>,
    pub end_position: usize,
    pub beatable_position: Option<usize>,
}

impl Iterator for Action {
    type Item = (Option<usize>, usize, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_position.is_some() {
            let start_position = self.start_position.take().unwrap();
            Some((Some(start_position), self.end_position, self.beatable_position.take()))
        } else {
            None
        }
    }
}

impl Action {
    pub fn new(start_position: Option<usize>, end_position: usize, beatable_position: Option<usize>) -> Self {
        Action { start_position, end_position, beatable_position }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        let mut encoded_string = String::new();
        if self.start_position.is_none() {
            encoded_string.push('P');
            encoded_string.push(' ');
            
            encoded_string.push_str(encode_single_position(self.end_position).as_str());
        } else {
            encoded_string.push('M');
            encoded_string.push(' ');
            
            encoded_string.push_str(encode_single_position(self.start_position.unwrap()).as_str());
            encoded_string.push(' ');
            encoded_string.push_str(encode_single_position(self.end_position).as_str());
        };
        
        if self.beatable_position.is_some() {
            encoded_string.push(' ');
            encoded_string.push('T');
            encoded_string.push(' ');
            encoded_string.push_str(encode_single_position(self.beatable_position.unwrap()).as_str());
        }
    
        return encoded_string;
    }
}


