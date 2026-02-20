use near_workspaces::{Contract, DevNetwork, Worker};

async fn init(worker: &Worker<impl DevNetwork>) -> anyhow::Result<(Contract, Contract)> {
    let adder = worker
        .dev_deploy(include_bytes!("../res/adder.wasm"))
        .await?;
    let delegator = worker
        .dev_deploy(include_bytes!("../res/delegator_macro.wasm"))
        .await?;
    Ok((adder, delegator))
}

#[tokio::test]
async fn test_delegate() -> anyhow::Result<()> {
    let worker = near_workspaces::sandbox().await?;
    let (adder, delegator) = init(&worker).await?;

    let res = delegator
        .call("delegate")
        .args_json((1u32, 2u32, 3u32, 4u32, adder.as_account().id()))
        .transact()
        .await?;
    assert_eq!(res.json::<(u32, u32)>()?, (4, 6));

    Ok(())
}
