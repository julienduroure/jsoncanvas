use std::path::PathBuf;

use crate::NodeId;
use crate::{color::Color, PixelCoordinate, PixelDimension};

use super::ambassador_impl_GenericNodeInfo;
use super::{GenericNode, GenericNodeInfo};
use ambassador::Delegate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Delegate, Serialize, Deserialize)]
#[delegate(GenericNodeInfo, target = "generic")]
pub struct FileNode {
    #[serde(flatten)]
    generic: GenericNode,
    file: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    subpath: Option<String>,
}

impl FileNode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: NodeId,
        x: PixelCoordinate,
        y: PixelCoordinate,
        width: PixelDimension,
        height: PixelDimension,
        color: Option<Color>,
        file: PathBuf,
        subpath: Option<String>,
    ) -> Self {
        Self {
            generic: GenericNode::new(id, x, y, width, height, color),
            file,
            subpath,
        }
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn subpath(&self) -> Option<&String> {
        self.subpath.as_ref()
    }
}
