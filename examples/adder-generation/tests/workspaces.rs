use workspaces::prelude::*;
use workspaces::{Contract, DevNetwork, Worker};

async fn init(worker: &Worker<impl DevNetwork>) -> anyhow::Result<(Contract, Contract)> {
    let adder = worker
        .dev_deploy(&include_bytes!("../res/adder.wasm").to_vec())
        .await?;
    let delegator = worker
        .dev_deploy(&include_bytes!("../res/delegator.wasm").to_vec())
        .await?;
    Ok((adder, delegator))
}

#[tokio::test]
async fn test_delegate() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let (adder, delegator) = init(&worker).await?;

    let res = delegator
        .call(&worker, "delegate")
        .args_json((1u32, 2u32, 3u32, 4u32, adder.as_account().id()))?
        .transact()
        .await?;
    assert_eq!(res.json::<(u32, u32)>()?, (4, 6));

    Ok(())
}
