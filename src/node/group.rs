use std::path::PathBuf;

use ambassador::Delegate;
use serde::{Deserialize, Serialize};

use crate::NodeId;
use crate::{color::Color, PixelCoordinate, PixelDimension};

use super::ambassador_impl_GenericNodeInfo;
use super::{GenericNode, GenericNodeInfo};

#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct GroupNode {
    #[serde(flatten)]
    generic: GenericNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    background: Option<Background>,
}

impl GroupNode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        label: Option<String>,
        background: Option<Background>,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            label,
            background,
        }
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn background(&self) -> Option<&Background> {
        self.background.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    image: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<BackgroundStyle>,
}

impl Background {
    pub fn new(image: PathBuf, background_style: Option<BackgroundStyle>) -> Background {
        Background {
            image,
            background_style,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundStyle {
    Cover,
    Ratio,
    Repeat,
}
