
use crate::{
    parse::split::Split,
};

#[derive(Debug)]
pub struct Source<'a> {
    //There will be multiple inputs once #import is implemented.
    pub input: Input<'a>,
}

impl<'a> Source<'a> {
    
    pub fn new(split: &Split) -> Self {
        todo!()    
    }

}

#[derive(Debug)]
pub struct Input<'a> {
    pub path: &'a str,
    pub content: String,
}
