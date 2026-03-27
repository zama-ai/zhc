use super::*;

/// Text element representing an operation input port.
pub type OpInputPort = TextBox<OpInputPortClass>;

/// Horizontal collection of input ports.
pub type OpInputs = HStack<OpInputPort, OpInputsClass>;

/// Text element representing an operation body.
pub type OpBody = TextBox<OpBodyClass>;

/// Text element representing an operation comment.
pub type OpComment = TextBox<OpCommentClass>;

/// Text element representing an operation output port.
pub type OpOutputPort = TextBox<OpOutputPortClass>;

/// Horizontal collection of output ports.
pub type OpOutputs = HStack<OpOutputPort, OpOutputsClass>;

/// Input operation: either without comment (Body, Outputs) or with comment (Body, Comment,
/// Outputs).
pub type InputOp = V3<OpBody, Optional<OpComment>, OpOutputs, InputOpClass>;

/// Standard operation: either without comment or with comment.
pub type Op = V4<OpInputs, OpBody, Optional<OpComment>, OpOutputs, OpClass>;

/// Effect operation: either without comment or with comment.
pub type EffectOp = V3<OpInputs, OpBody, Optional<OpComment>, EffectOpClass>;

/// Empty placeholder element for missing nodes.
pub type Dummy = Empty<DummyClass>;

/// Text element representing a group input boundary port.
pub type GroupInputPort = Empty<GroupInputPortClass>;

/// Horizontal collection of group input ports.
pub type GroupInputs = HStack<GroupInputPort, GroupInputsClass>;

/// Text element representing a group output boundary port.
pub type GroupOutputPort = Empty<GroupOutputPortClass>;

/// Horizontal collection of group output ports.
pub type GroupOutputs = HStack<GroupOutputPort, GroupOutputsClass>;

/// Text element representing a group title.
pub type GroupTitle = TextBox<GroupTitleClass>;

/// Group element containing nested vertices with boundary ports.
pub struct Group(pub V4<GroupTitle, GroupInputs, GroupContent, GroupOutputs, GroupClass>);

impl Element for Group {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        self.0.solve_size(stylesheet);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: zhc_utils::graphics::Frame) {
        self.0.solve_frame(stylesheet, available);
    }

    fn get_size(&self) -> zhc_utils::graphics::Size {
        self.0.get_size()
    }

    fn get_frame(&self) -> zhc_utils::graphics::Frame {
        self.0.get_frame()
    }

    fn get_variable_cell(&self) -> VariableCell {
        self.0.get_variable_cell()
    }
}

pub type Node = D7<InputOp, Op, EffectOp, Dummy, Group, GroupInputPort, GroupOutputPort>;
pub use D7::E1 as NodeInputOpVar;
pub use D7::E2 as NodeOpVar;
pub use D7::E3 as NodeEffectOpVar;
pub use D7::E4 as NodeDummyVar;
pub use D7::E5 as NodeGroupVar;
pub use D7::E6 as NodeGroupInputPortVar;
pub use D7::E7 as NodeGroupOutputPortVar;

/// Horizontal row of nodes forming a diagram layer.
pub type Layer = HStack<Node, LayerClass>;

pub type LayerSeparator = Spacer<LayerClass>;

pub type LayerMember = D2<Layer, LayerSeparator>;
pub use D2::E1 as LayerMemberLayer;
pub use D2::E2 as LayerMemberSeparator;

/// All the diagram layers
pub type Layers = VStack<LayerMember, VerticesClass>;

/// Content inside a group element (uses smaller padding/spacing than top-level Vertices).
pub type GroupContent = Layers;
