use crate::reexport::{Anchor, Layer};
use enative_core::window::Id as WindowId;
use enative_core::mouse::Interaction;
use layershellev::id::Id as LayerId;
use layershellev::NewLayerShellSettings;

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) enum LayerShellActions<INFO: Clone> {
    Mouse(Interaction),
    CustomActions(LayershellCustomActionsWithInfo<INFO>),
    CustomActionsWithId(LayershellCustomActionsWithIdInner<INFO>),
    RedrawAll,
    RedrawWindow(LayerId),
    NewMenu((LayershellNewPopupSettings, INFO)),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LayershellNewPopupSettings {
    pub size: (u32, u32),
    pub position: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuDirection {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LayershellNewMenuSettings {
    pub size: (u32, u32),
    pub direction: MenuDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum LayershellCustomActionsWithInfo<INFO: Clone> {
    AnchorChange(Anchor),
    LayerChange(Layer),
    MarginChange((i32, i32, i32, i32)),
    SizeChange((u32, u32)),
    VirtualKeyboardPressed {
        time: u32,
        key: u32,
    },
    NewLayerShell((NewLayerShellSettings, INFO)),
    NewPopUp((LayershellNewPopupSettings, INFO)),
    NewMenu((LayershellNewMenuSettings, INFO)),
    RemoveWindow(WindowId),
    ForgetLastOutput,
}

pub type LayershellCustomActions = LayershellCustomActionsWithInfo<()>;

#[derive(Debug, Clone, Copy)]
pub struct LayershellCustomActionsWithIdAndInfo<INFO: Clone>(
    pub Option<WindowId>,
    pub LayershellCustomActionsWithInfo<INFO>,
);

impl<INFO: Clone> LayershellCustomActionsWithIdAndInfo<INFO> {
    pub fn new(id: Option<WindowId>, actions: LayershellCustomActionsWithInfo<INFO>) -> Self {
        Self(id, actions)
    }
}

pub type LayershellCustomActionsWithId = LayershellCustomActionsWithIdAndInfo<()>;

#[derive(Debug, Clone, Copy)]
pub(crate) struct LayershellCustomActionsWithIdInner<INFO: Clone>(
    pub Option<LayerId>,
    pub Option<LayerId>,
    pub LayershellCustomActionsWithInfo<INFO>,
);
