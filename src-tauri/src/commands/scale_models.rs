#[cfg(any(feature = "desktop", feature = "mobile"))]
use crate::{app_state::SharedState, db::open_connection};
use super::scale_simulator::{scale_simulator_preview_frame, ScaleSimulatorConfig};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "desktop", feature = "mobile"))]
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct SavedScaleModel {
    id: String,
    name: String,
    config: ScaleSimulatorConfig,
    revision: i64,
}
pub(crate) fn schema(db: &Connection) -> Result<(), String> {
    db.execute_batch("CREATE TABLE IF NOT EXISTS scale_simulation_templates (id TEXT PRIMARY KEY, name TEXT NOT NULL COLLATE NOCASE UNIQUE, config_json TEXT NOT NULL, revision INTEGER NOT NULL DEFAULT 1);")
        .map_err(|e| format!("Não foi possível abrir os modelos: {e}"))
}
pub(crate) fn list(db: &Connection) -> Result<Vec<SavedScaleModel>, String> {
    schema(db)?;
    let mut statement = db.prepare("SELECT id,name,config_json,revision FROM scale_simulation_templates ORDER BY name COLLATE NOCASE,id").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(|e| e.to_string())?;
    rows.map(|row| {
        let (id, name, json, revision) = row.map_err(|e| e.to_string())?;
        let mut config: ScaleSimulatorConfig =
            serde_json::from_str(&json).map_err(|e| e.to_string())?;
        config.id = None;
        Ok(SavedScaleModel {
            id,
            name,
            config,
            revision,
        })
    })
    .collect()
}
pub(crate) fn save(
    db: &Connection,
    id: Option<String>,
    name: String,
    mut config: ScaleSimulatorConfig,
    expected_revision: Option<i64>,
) -> Result<SavedScaleModel, String> {
    schema(db)?;
    let name = name.trim().to_string();
    if name.is_empty() || name.chars().count() > 80 {
        return Err("Informe um nome de até 80 caracteres.".into());
    }
    config.id = None;
    // Pure validation/preview, never opens a port or starts/stops an instance.
    scale_simulator_preview_frame(config.clone())?;
    let json = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    if json.len() > 32768 {
        return Err("Modelo excede o limite de tamanho.".into());
    }
    let updating = id.is_some();
    let id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let revision = if updating {
        expected_revision
            .filter(|r| *r > 0 && *r < i64::MAX)
            .ok_or("Recarregue o modelo antes de atualizar.")?
    } else {
        0
    };
    let result = if updating {
        db.execute("UPDATE scale_simulation_templates SET name=?1,config_json=?2,revision=revision+1 WHERE id=?3 AND revision=?4",params![name,json,id,revision])
    } else {
        db.execute("INSERT INTO scale_simulation_templates(id,name,config_json,revision) VALUES(?1,?2,?3,1)",params![id,name,json])
    };
    match result {
        Ok(1) => Ok(SavedScaleModel {
            id,
            name,
            config,
            revision: revision + 1,
        }),
        Ok(_) => Err("O modelo foi alterado ou removido. Recarregue a lista.".into()),
        Err(e) => Err(format!(
            "Não foi possível salvar. Verifique se o nome já existe. {e}"
        )),
    }
}
#[cfg(any(feature = "desktop", feature = "mobile"))]
#[tauri::command]
pub fn scale_models_list(state: State<SharedState>) -> Result<Vec<SavedScaleModel>, String> {
    list(&open_connection(&state.db_path()?)?)
}
#[cfg(any(feature = "desktop", feature = "mobile"))]
#[tauri::command]
pub fn scale_model_save(
    state: State<SharedState>,
    id: Option<String>,
    name: String,
    config: ScaleSimulatorConfig,
    expected_revision: Option<i64>,
) -> Result<SavedScaleModel, String> {
    save(
        &open_connection(&state.db_path()?)?,
        id,
        name,
        config,
        expected_revision,
    )
}
#[cfg(any(feature = "desktop", feature = "mobile"))]
#[tauri::command]
pub fn scale_model_delete(
    state: State<SharedState>,
    id: String,
    expected_revision: i64,
) -> Result<(), String> {
    let db = open_connection(&state.db_path()?)?;
    schema(&db)?;
    let count = db
        .execute(
            "DELETE FROM scale_simulation_templates WHERE id=?1 AND revision=?2",
            params![id, expected_revision],
        )
        .map_err(|e| e.to_string())?;
    if count != 1 {
        return Err("O modelo mudou. Recarregue antes de remover.".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> ScaleSimulatorConfig {
        serde_json::from_value(serde_json::json!({"id":"active-instance","name":"Balança","driver":"generic_ascii","mode":"tcp_server","host":"127.0.0.1","port":9101,"serial_port":"COM99","baud_rate":9600,"capacity_kg":80000.0,"division_kg":1.0,"initial_weight_kg":1234.0,"variation_kg":0.0,"interval_ms":500,"stable":true,"zero_tracking":true,"unit":"kg"})).unwrap()
    }
    #[test]
    fn models_are_independent_and_revision_guarded() {
        let db = Connection::open_in_memory().unwrap();
        let a = save(&db, None, "A".into(), config(), None).unwrap();
        let mut other = config();
        other.serial_port = "COM100".into();
        let b = save(&db, None, "B".into(), other, None).unwrap();
        assert_ne!(a.id, b.id);
        assert!(a.config.id.is_none());
        assert_eq!(list(&db).unwrap().len(), 2);
        let updated = save(
            &db,
            Some(a.id.clone()),
            "A".into(),
            config(),
            Some(a.revision),
        )
        .unwrap();
        assert_eq!(updated.revision, 2);
        assert!(save(&db, Some(a.id), "A".into(), config(), Some(1)).is_err());
        assert!(save(&db, None, "a".into(), config(), None).is_err());
    }
}
