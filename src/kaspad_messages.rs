use crate::{
    pow::{self, HeaderHasher},
    proto::{
        kaspad_request::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage, KaspadRequest,
        NotifyBlockAddedRequestMessage, NotifyNewBlockTemplateRequestMessage, RpcBlock, RpcNotifyCommand,
        SubmitBlockRequestMessage,
    },
    Hash,
};

impl KaspadRequest {
    #[must_use]
    #[inline(always)]
    pub fn get_info_request() -> Self {
        KaspadRequest { id: 1063, payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }

    #[must_use]
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        KaspadRequest {
            id: 1007,
            payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {
                command: RpcNotifyCommand::NotifyStart as i32,
            })),
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        KaspadRequest {
            id: 1003,
            payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage {
                block: Some(block),
                allow_non_daa_blocks: false,
            })),
        }
    }
}

impl From<GetInfoRequestMessage> for KaspadRequest {
    #[inline(always)]
    fn from(a: GetInfoRequestMessage) -> Self {
        KaspadRequest { id: 1063, payload: Some(Payload::GetInfoRequest(a)) }
    }
}

impl From<NotifyBlockAddedRequestMessage> for KaspadRequest {
    #[inline(always)]
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        KaspadRequest { id: 1007, payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for KaspadRequest {
    #[inline(always)]
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        KaspadRequest { id: 1005, payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl From<NotifyNewBlockTemplateRequestMessage> for KaspadRequest {
    #[inline(always)]
    fn from(a: NotifyNewBlockTemplateRequestMessage) -> Self {
        KaspadRequest { id: 1081, payload: Some(Payload::NotifyNewBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[must_use]
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
