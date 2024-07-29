use crate::renderer::draw::Draw;

pub struct DrawingResource(pub Box<dyn Draw>);
