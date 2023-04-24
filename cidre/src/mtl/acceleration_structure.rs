use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Usage {
    ///  Default usage
    None = 0,

    /// Enable refitting for this acceleration structure. Note that this may reduce
    /// acceleration structure quality.
    Refit = (1 << 0),

    /// Prefer building this acceleration structure quickly at the cost of reduced ray
    /// tracing performance.
    PreferFastBuild = (1 << 1),

    /// Enable extended limits for this acceleration structure, possibly at the cost of
    /// reduced ray tracing performance.
    ExtendedLimits = (1 << 2),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum InstanceOptions {
    /// No options
    None = 0,

    /// Disable triangle back or front face culling
    DisableTriangleCulling = (1 << 0),

    /// Disable triangle back or front face culling
    TriangleFrontFacingWindingCounterClockwise = (1 << 1),

    /// Geometry is opaque
    Opaque = (1 << 2),

    /// Geometry is non-opaque
    NonOpaque = (1 << 3),
}

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    #[objc::msg_send(usage)]
    pub fn usage(&self) -> Usage;

    #[objc::msg_send(setUsage:)]
    pub fn set_usage(&mut self, value: Usage);
}

define_obj_type!(
    GeometryDescriptor(Descriptor),
    MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR
);

impl GeometryDescriptor {
    #[objc::msg_send(intersectionFunctionTableOffset)]
    pub fn intersection_fn_table_offset(&self) -> usize;

    #[objc::msg_send(setIntersectionFunctionTableOffset:)]
    pub fn set_intersection_fn_table_offset(&mut self, value: usize);

    #[objc::msg_send(opaque)]
    pub fn opaque(&self) -> bool;

    #[objc::msg_send(setOpaque:)]
    pub fn set_opaque(&mut self, value: bool);

    #[objc::msg_send(allowDuplicateIntersectionFunctionInvocation)]
    pub fn allow_duplicate_intersection_fn_invocation(&self) -> bool;

    #[objc::msg_send(setAllowDuplicateIntersectionFunctionInvocation:)]
    pub fn set_allow_duplicate_intersection_fn_invocation(&mut self, value: bool);

    define_mtl!(label, set_label);

    #[objc::msg_send(primitiveDataBuffer)]
    pub fn primitive_data_buf(&self) -> Option<&mtl::Buf>;

    #[objc::msg_send(setPrimitiveDataBuffer:)]
    pub fn set_primitive_data_buf(&mut self, value: Option<&mtl::Buf>);

    #[objc::msg_send(primitiveDataBufferOffset)]
    pub fn primitive_data_buf_offset(&self) -> usize;

    #[objc::msg_send(setPrimitiveDataBufferOffset:)]
    pub fn set_primitive_data_buf_offset(&mut self, value: usize);

    #[objc::msg_send(primitiveDataStride)]
    pub fn primitive_data_stride(&self) -> usize;

    #[objc::msg_send(setPrimitiveDataStride:)]
    pub fn set_primitive_data_stride(&mut self, value: usize);

    #[objc::msg_send(primitiveDataElementSize)]
    pub fn primitive_data_element_size(&self) -> usize;

    #[objc::msg_send(setPrimitiveDataElementSize:)]
    pub fn set_primitive_data_element_size(&self, value: usize);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum MotionBorderMode {
    /// Motion is stopped. (default)
    Clamp = 0,

    /// Object disappears
    Vanish = 1,
}

extern "C" {
    static MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR: &'static objc::Class<Descriptor>;
}
