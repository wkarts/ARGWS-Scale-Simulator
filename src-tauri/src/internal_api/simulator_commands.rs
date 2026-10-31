use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use super::state::InternalApiState;
use crate::{
    commands::{auth::authorize_api_command, scale_models, scale_simulator::*},
    db::open_connection,
};
fn field<T: DeserializeOwned>(args: &Map<String, Value>, name: &str) -> Result<T, String> {
    serde_json::from_value(args.get(name).cloned().unwrap_or(Value::Null))
        .map_err(|_| format!("Parâmetro inválido: {name}"))
}
fn value<T: serde::Serialize>(result: T) -> Result<Value, String> {
    serde_json::to_value(result).map_err(|e| e.to_string())
}
pub async fn execute(
    state: &InternalApiState,
    command: &str,
    args: Map<String, Value>,
) -> Result<Value, String> {
    {
        let db = open_connection(&state.db_path)?;
        let token = field::<String>(&args, "session_token")?;
        authorize_api_command(&db, &token, command)?;
    }
    match command {
        "scale_simulator_serial_ports" => value(scale_simulator_serial_ports()?),
        "scale_simulator_presets" => value(scale_simulator_presets()),
        "scale_simulator_preview_frame" => {
            value(scale_simulator_preview_frame(field(&args, "config")?)?)
        }
        "scale_simulator_start" => value(scale_simulator_start(field(&args, "config")?).await?),
        "scale_simulator_stop" => value(scale_simulator_stop(field(&args, "id")?)?),
        "scale_simulator_stop_all" => value(scale_simulator_stop_all()?),
        "scale_simulator_list" => value(scale_simulator_list()?),
        "scale_simulator_logs" => value(scale_simulator_logs(field(&args, "id")?)?),
        "scale_simulator_set_weight" => value(scale_simulator_set_weight(
            field(&args, "id")?,
            field(&args, "weight_kg")?,
        )?),
        "scale_simulator_action" => value(scale_simulator_action(
            field(&args, "id")?,
            field(&args, "action")?,
            field(&args, "value")?,
        )?),
        "scale_models_list" => value(scale_models::list(&open_connection(&state.db_path)?)?),
        "scale_model_save" => value(scale_models::save(
            &open_connection(&state.db_path)?,
            field(&args, "id")?,
            field(&args, "name")?,
            field(&args, "config")?,
            field(&args, "expected_revision")?,
        )?),
        "scale_model_delete" => {
            let db = open_connection(&state.db_path)?;
            scale_models::schema(&db)?;
            let changed = db
                .execute(
                    "DELETE FROM scale_simulation_templates WHERE id=?1 AND revision=?2",
                    rusqlite::params![
                        field::<String>(&args, "id")?,
                        field::<i64>(&args, "expected_revision")?
                    ],
                )
                .map_err(|e| e.to_string())?;
            if changed != 1 {
                return Err("Modelo alterado ou removido. Recarregue.".into());
            }
            Ok(Value::Null)
        }
        _ => Err("Comando de simulação desconhecido.".into()),
    }
}
