use goose::prelude::*;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
      .register_scenario(scenario!("LoadtestTransactions")
        .register_transaction(transaction!(loadtest_predict))
      )
      .execute()
      .await?;
      
      Ok(())

}


async fn loadtest_predict(user: &mut GooseUser) -> TransactionResult {
	let json = &serde_json::json!({
			"cart_position": -0.04568531,
			"cart_velocity": -0.14921477,
			"pole_angle": 0.01811073,
			"pole_angular_velocity": 0.28565928,
	});
  let _goose = user.post_json("predict", &json).await?;

  Ok(())
}
