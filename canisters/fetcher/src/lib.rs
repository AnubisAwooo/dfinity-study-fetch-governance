mod constants;
use constants::{GOVERNANCE_CANISTER_ID, PROCESS_INTERVAL};

mod types;
use types::{HttpResponse, ListProposalInfo, ListProposalInfoResponse, ProposalInfo};

use ic_cdk::api::call;
use ic_cdk::api::time;
use ic_cdk_macros::{heartbeat, query};

use std::iter::FromIterator;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static LAST_UPDATED: RefCell<u64> = RefCell::new(0); // 上次更新时间
    static LAST_PROPOSALS: RefCell<Vec<ProposalInfo>> = RefCell::new(vec![]); // 上次保存的最近 100 条提案信息
}

// 测试函数 获取最新的提案 id
#[query]
fn last_proposal_id() -> u64 {
    LAST_PROPOSALS.with(|list| {
        let list = list.borrow();
        match list.first() {
            Some(info) => match &info.id {
                Some(id) => id.id,
                None => 0,
            },
            None => 0,
        }
    })
}

// 获取最近更新的时间 毫秒
#[query]
fn last_updated() -> u64 {
    LAST_UPDATED.with(|u| u.borrow().clone())
}

// 异步查询最近的 limit 条提案信息
#[ic_cdk_macros::update]
async fn last_proposals(limit: u32) -> Vec<ProposalInfo> {
    let r = list_proposals(limit).await;
    match r {
        Ok(v) => v,
        Err(_) => vec![],
    }
}

// 获取提案信息 异步调用治理提案的接口
async fn list_proposals(limit: u32) -> Result<Vec<ProposalInfo>, String> {
    let call_result: Result<(ListProposalInfoResponse,), _> = ic_cdk::api::call::call(
        ic_cdk::export::Principal::from_text(GOVERNANCE_CANISTER_ID).unwrap(),
        "list_proposals", // 调用 rrkah-fqaaa-aaaaa-aaaaq-cai 的 list_proposals 方法
        (ListProposalInfo {
            include_reward_status: vec![], // 过滤参数 包含的奖励状态
            before_proposal: None,         // 过滤参数 之前的提案 id
            limit,                         // 过滤参数 查询个数
            exclude_topic: vec![],         // 过滤参数 排除主题
            include_status: vec![],        // 过滤参数 包含状态
        },),
    )
    .await;

    match call_result {
        Ok(value) => Ok(value.0.proposal_info),
        Err(error) => Err(error.1),
    }
}

// 定时触发更新数据
#[heartbeat]
fn tick() {
    LAST_UPDATED.with(|updated| {
        let mut updated = updated.borrow_mut();
        let now = time();
        if *updated + PROCESS_INTERVAL < now {
            // 已经过去 PROCESS_INTERVAL 纳秒了，那么就更新数据
            *updated = now; // 先更新时间，防止再次进入
                            // ? 开启一个线程执行？
            ic_cdk::spawn(async {
                let list = last_proposals(100).await; // 获取最新的 100 条数据
                LAST_PROPOSALS.with(move |proposals| {
                    let mut proposals = proposals.borrow_mut();
                    proposals.clear();
                    proposals.extend_from_slice(&list[..]); // 更新数据
                });
            });
        }
    })
}

// 直接返回缓存中保存的数据
#[query]
fn last_proposals2() -> Vec<ProposalInfo> {
    LAST_PROPOSALS.with(|v| v.borrow().clone())
}

// http 请求获取最新数据
#[export_name = "canister_query http_request"]
fn http_request() {
    ic_cdk::setup();
    LAST_PROPOSALS.with(|list| {
        let list = list.borrow();
        let r: &Vec<ProposalInfo> = list.as_ref();
        let s: String = serde_json::to_string(r).unwrap();
        let s: &[u8] = s.as_bytes();
        let body = s[..].into();
        call::reply((HttpResponse {
            status_code: 200,
            headers: HashMap::from_iter([]),
            body,
        },));
    });
}
