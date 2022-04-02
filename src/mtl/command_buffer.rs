use std::ffi::c_void;

use crate::cf::runtime::Autoreleased;
use crate::{define_mtl, define_obj_type, msg_send};

use crate::ns::Id;
use crate::objc::block::CompletionHandlerA;

use super::{BlitCommandEncoder, CommandQueue, ComputeCommandEncoder};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Status {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Error {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    StackOverflow = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum DispatchType {
    Serial,
    Concurrent,
}

define_obj_type!(CommandBuffer(Id));

impl CommandBuffer {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[inline]
    pub fn command_queue(&self) -> &CommandQueue {
        msg_send!(self, sel_commandQueue)
    }

    #[inline]
    pub fn enqueue(&self) {
        msg_send!(self, sel_enqueue)
    }

    #[inline]
    pub fn commit(&self) {
        msg_send!(self, sel_commit)
    }

    #[inline]
    pub fn wait_untint_scheduled(&self) {
        msg_send!(self, sel_waitUntilScheduled)
    }

    #[inline]
    pub fn wait_until_completed(&self) {
        msg_send!(self, sel_waitUntilCompleted)
    }

    pub fn add_scheduled_handler<B>(&self, block: B)
    where
        B: FnOnce(&Self) + Send + 'static,
    {
        unsafe { sel_addScheduledHandler(self, block.into_raw()) }
    }

    pub fn add_completion_handler<B>(&self, block: B)
    where
        B: FnOnce(&Self) + Send + 'static,
    {
        unsafe { sel_addCompletedHandler(self, block.into_raw()) }
    }

    #[inline]
    pub fn blit_command_encoder<'new>(&self) -> Option<Autoreleased<'new, BlitCommandEncoder>> {
        msg_send!(self, sel_blitCommandEncoder)
    }

    #[inline]
    pub fn compute_command_encoder<'new>(
        &self,
    ) -> Option<Autoreleased<'new, ComputeCommandEncoder>> {
        msg_send!(self, sel_computeCommandEncoder)
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn sel_addScheduledHandler(id: &Id, rb: *const c_void);
    fn sel_addCompletedHandler(id: &Id, rb: *const c_void);
}
