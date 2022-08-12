use windows::Data::Xml::Dom::XmlElement;

use crate::hs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationType {
    Foreground,
    Protocol,
}

impl ActivationType {
    fn as_str(&self) -> &'static str {
        match self {
            ActivationType::Foreground => "foreground",
            ActivationType::Protocol => "protocol",
        }
    }
}

/// See https://docs.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/toast-headers
#[derive(Debug, Clone)]
pub struct Header {
    id: String,
    title: String,
    arguments: String,
    activation_type: Option<ActivationType>,
}

impl Header {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        arguments: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            arguments: arguments.into(),
            activation_type: None,
        }
    }

    pub fn with_activation_type(mut self, activation_type: ActivationType) -> Self {
        self.activation_type = Some(activation_type);
        self
    }

    pub(crate) fn write_to_element(&self, el: &XmlElement) -> crate::Result<()> {
        el.SetAttribute(&hs("id"), &hs(&self.id))?;
        el.SetAttribute(&hs("title"), &hs(&self.title))?;
        el.SetAttribute(&hs("arguments"), &hs(&self.arguments))?;
        if let Some(activation_type) = self.activation_type {
            el.SetAttribute(&hs("activationType"), &hs(activation_type.as_str()))?;
        }

        Ok(())
    }
}
