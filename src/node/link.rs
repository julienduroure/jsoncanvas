use ambassador::Delegate;
use serde::{Deserialize, Serialize};
pub use url::Url;

use crate::NodeId;
use crate::{color::Color, PixelCoordinate, PixelDimension};

use super::ambassador_impl_GenericNodeInfo;
use super::{GenericNode, GenericNodeInfo};

#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct LinkNode {
    #[serde(flatten)]
    generic: GenericNode,
    url: Url,
}

impl LinkNode {
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        url: Url,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            url,
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
