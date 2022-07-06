use ic_cdk::export::{candid::CandidType, serde::Deserialize, serde::Serialize};

use std::borrow::Cow;
use std::collections::HashMap; // Clone on Write 写时克隆功能

// 神经元 id  是个 64 位的数字吗？
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct NeuronId {
    pub id: u64,
}

// 列出提案信息
#[derive(CandidType, Deserialize)]
pub struct ListProposalInfo {
    pub include_reward_status: Vec<i32>,   // 包含奖励状态 ？
    pub before_proposal: Option<NeuronId>, // 之前提案？
    pub limit: u32,                        // 限制？
    pub exclude_topic: Vec<i32>,           // 排除主题
    pub include_status: Vec<i32>,          // 包含状态
}

// 列出提案响应
#[derive(CandidType, Deserialize)]
pub struct ListProposalInfoResponse {
    pub proposal_info: Vec<ProposalInfo>,
}

// 提案信息
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct ProposalInfo {
    pub id: Option<NeuronId>,       // id 应该是数字，不应该是 NeuronId 才是
    pub topic: i32,                 // 提案主题
    pub proposal: Option<Proposal>, // 提案内容
}

// 提案结构体
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Proposal {
    pub url: String,           // url
    pub title: Option<String>, // 标题
    pub summary: String,       // 提案简介
}

// http

// http 请求体结构
#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

// http 响应体结构
#[derive(CandidType)]
pub struct HttpResponse<'a> {
    pub status_code: u16,
    pub headers: HashMap<&'a str, Cow<'a, str>>,
    pub body: Cow<'a, [u8]>,
}
