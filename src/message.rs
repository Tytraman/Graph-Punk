use std::collections::HashMap;

use crate::{renderer::Renderer, resource::Resource, types::UserData};

pub struct MessageCaller<'a> {
    messages: HashMap<
        String,
        (
            Box<dyn FnMut(&mut Renderer, &mut Resource, &mut UserData) + 'a>,
            Vec<UserData>,
        ),
    >,
}

impl<'a> Default for MessageCaller<'a> {
    fn default() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }
}

impl<'a> MessageCaller<'a> {
    pub fn register_message(
        &mut self,
        unique_id: &str,
        c: impl FnMut(&mut Renderer, &mut Resource, &mut UserData) + 'a,
    ) {
        self.messages
            .insert(unique_id.to_string(), (Box::new(c), Vec::new()));
    }

    pub fn add_message(&mut self, unique_id: &str, user_data: UserData) -> Result<(), String> {
        let message = self
            .messages
            .get_mut(unique_id)
            .ok_or("no message found".to_string())?;

        message.1.push(user_data);

        Ok(())
    }

    pub fn execute(&mut self, renderer: &mut Renderer, resource: &mut Resource) {
        self.messages.iter_mut().for_each(|(_, message)| {
            if !message.1.is_empty() {
                message
                    .1
                    .iter_mut()
                    .for_each(|user_data| (message.0)(renderer, resource, user_data));

                message.1.clear();
            }
        })
    }
}
